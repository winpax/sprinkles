use super::{Error, Result};

pub struct LibXML;

impl super::XMLProvider for LibXML {
    fn find_xpath(source: &str, xpath: &str) -> Result<String> {
        unimplemented!()
    }

    fn find_rdf(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String> {
        todo!()
    }
}
