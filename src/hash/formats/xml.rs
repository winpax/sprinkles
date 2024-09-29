use crate::hash::substitutions::{Substitute, SubstitutionMap};

pub(crate) mod provisions;

use provisions::{Provider, XMLProvider};

pub use provisions::Error as XMLError;

pub fn parse_xml(
    source: impl AsRef<str>,
    substitutions: &SubstitutionMap,
    xpath: impl AsRef<str>,
) -> Result<String, provisions::Error> {
    let xpath = xpath
        .as_ref()
        .to_string()
        .into_substituted(substitutions, false);

    Provider::find_xpath(source.as_ref(), xpath.as_ref())
}
