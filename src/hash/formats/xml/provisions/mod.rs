//! Providers for xml parsing

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::DeError),
    #[error("XML Parsing: {0}")]
    SXDXml(#[from] sxd_document::parser::Error),
    #[error("XML XPath: {0}")]
    XPath(#[from] sxd_xpath::Error),
    #[error("Hash not found")]
    NotFound,
    #[error("Invalid value")]
    InvalidValue,

    // LibXML Errors
    #[cfg(feature = "libxml")]
    #[error("XML parsing error: {0}")]
    Parsing(#[from] libxml::parser::XmlParseError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait XMLProvider {
    fn find_xpath(source: &str, xpath: &str) -> Result<String>;

    fn find_rdf(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "libxml")] {
        mod libxml_provider;
        pub use libxml_provider::LibXML as Provider;
    } else {
        pub use other::OtherProviders as Provider;
    }
}

mod other;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::hash::substitutions::SubstitutionMap;

    use super::*;

    #[rstest]
    #[case(other::OtherProviders)]
    #[cfg(feature = "libxml")]
    #[case(libxml_provider::LibXML)]
    fn test_finding_in_xml(#[case] provider: impl XMLProvider) -> anyhow::Result<()> {
        const EXAMPLE_XML: &str = r#"
            <assembly>
                <description>sfsu</description>
                <compatibility>
                    <application>
                        <!-- Windows 10 and Windows 11 -->
                        <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}" />
                        <!-- Windows 8.1 -->
                        <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}" />
                        <!-- Windows 8 -->
                        <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}" />
                        <!-- Windows 7 -->
                        <supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}" />
                        <!-- Windows Vista -->
                        <supportedOS Id="{e2011457-1546-43c5-a5fe-008deee3d3f0}" />
                    </application>
                </compatibility>
            </assembly>
        "#;

        let mut submap = SubstitutionMap::new();

        submap.insert("$finalKey".to_string(), "supportedOS".to_string());

        let hash = parse_xml(
            EXAMPLE_XML,
            &submap,
            "/assembly/compatibility/application/$finalKey[last() - 1]/@Id",
        )?;

        assert_eq!(hash, "{35138b9a-5d96-4fbd-8e2d-a2440225f93a}");

        Ok(())
    }
}
