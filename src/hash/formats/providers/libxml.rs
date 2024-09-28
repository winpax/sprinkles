pub struct LibXML;

impl super::XMLProvider for LibXML {
    fn find_xpath(source: &str, xpath: &str) -> Option<String> {
        unimplemented!()
    }
}
