use libxml::{parser::Parser, xpath::Context};

use super::{Error, Result};

pub struct LibXML;

impl super::XMLProvider for LibXML {
    fn find_xpath(source: &str, xpath: &str) -> Result<String> {
        let parser = Parser::default();
        let doc = parser.parse_string(source)?;

        let xpath_context =
            Context::new(&doc).expect("valid document. found internal libxml2 error");

        if let Ok(obj) = xpath_context.evaluate(xpath) {
            let matches = obj.get_nodes_as_str();
            matches.into_iter().last().ok_or(Error::NotFound)
        } else {
            Err(Error::NotFound)
        }
    }

    fn find_rdf(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String> {
        todo!()
    }
}
