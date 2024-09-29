pub use super::xml::provisions::Error as RDFError;
use super::xml::provisions::XMLProvider;

pub fn parse_xml(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String, RDFError> {
    super::xml::provisions::Provider::find_rdf(input, file_name)
}
