use crate::hash::substitutions::Substitute;

pub struct XPath {
    pub(super) xpath: String,
}

impl From<String> for XPath {
    fn from(value: String) -> Self {
        Self { xpath: value }
    }
}

impl From<XPath> for String {
    fn from(value: XPath) -> Self {
        value.xpath
    }
}

impl Substitute for XPath {
    fn substitute(
        &mut self,
        params: &crate::hash::substitutions::SubstitutionMap,
        regex_escape: bool,
    ) {
        self.xpath.substitute(params, regex_escape);
    }
}
