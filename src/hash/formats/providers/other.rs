pub fn find_xpath(source: &str, xpath: &str) -> Option<String> {
    let pkg = parser::parse(source.as_ref())?;
    let doc = pkg.as_document();

    let value = evaluate_xpath(&doc, xpath.as_ref())?;

    let hash = match value {
        Value::Nodeset(nodes) => {
            let node = nodes.iter().last().ok_or(XMLError::NotFound)?;

            node.string_value()
        }
        Value::String(text) => text,
        _ => return Err(XMLError::InvalidValue),
    };

    Ok(hash)
}
