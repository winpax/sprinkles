mod quick;
mod sxd;

use super::Result;

pub struct OtherProviders;

impl super::XMLProvider for OtherProviders {
    fn find_xpath(source: &str, xpath: &str) -> Result<String> {
        sxd::find_xpath(source, xpath)
    }

    fn find_rdf(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String> {
        quick::find_rdf(input, file_name)
    }
}
