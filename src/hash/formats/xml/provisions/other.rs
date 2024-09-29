mod sxd;

use super::Result;

pub struct OtherProviders;

impl super::XMLProvider for OtherProviders {
    fn find_xpath(source: &str, xpath: &str) -> Result<String> {
        sxd::find_xpath(source, xpath)
    }
}
