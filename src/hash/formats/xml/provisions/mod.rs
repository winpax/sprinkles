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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait XMLProvider {
    fn find_xpath(source: &str, xpath: &str) -> Result<String>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "libxml")] {
        mod libxml;
        pub use libxml::LibXML as Provider;
    } else {
        pub use other::Other as Provider;
    }
}

mod other;
