use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use sxd_document::parser;
use sxd_xpath::{evaluate_xpath, Value};

fn xpaths(c: &mut Criterion) {
    const XML_FILE: &str = include_str!("../tests/fixtures/canada.xml");
    const XPATH: &str = "/root/features/geometry/coordinates/entry";

    #[cfg(feature = "libxml")]
    c.bench_function("libxml xpath", |b| {
        use libxml::{parser::Parser, xpath::Context};

        b.iter(|| {
            let parser = Parser::default();
            let doc = parser.parse_string(black_box(XML_FILE)).unwrap();

            let context = Context::new(&doc).expect("valid document. found internal libxml2 error");

            let obj = context.evaluate(black_box(XPATH)).unwrap();
            let matches = obj.get_nodes_as_str();
            matches.into_iter().last().unwrap();
        });
    });

    c.bench_function("sxd xpath", |b| {
        b.iter(|| {
            let pkg = parser::parse(black_box(XML_FILE)).unwrap();
            let doc = pkg.as_document();

            let value = evaluate_xpath(&doc, black_box(XPATH)).unwrap();

            match value {
                Value::Nodeset(nodes) => {
                    let node = nodes.iter().last().unwrap();

                    node.string_value()
                }
                Value::String(text) => text,
                _ => panic!("Invalid value"),
            };
        });
    });
}

criterion_group!(benches, xpaths);
criterion_main!(benches);
