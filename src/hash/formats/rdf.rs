pub use super::xml::provisions::Error as RDFError;
use super::xml::provisions::XMLProvider;

pub fn parse_xml(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String, RDFError> {
    super::xml::provisions::Provider::find_rdf(input, file_name)
}

#[cfg(test)]
mod tests {
    use crate::requests::Client;

    use super::*;

    #[test]
    #[ignore = "imagemagick website connection timing out"]
    pub fn test_finding_imagemagick_hashes() {
        const RDF_URL: &str = "https://download.imagemagick.org/archive/binaries/digest.rdf";

        let rdf_file = Client::blocking()
            .get(RDF_URL)
            .send()
            .unwrap()
            .text()
            .unwrap();

        for file_name in [
            "ImageMagick-i686-pc-cygwin.tar.gz",
            "ImageMagick-i386-pc-solaris2.11.tar.gz",
        ] {
            let hash = parse_xml(&rdf_file, file_name).unwrap();

            match file_name {
                "ImageMagick-i686-pc-cygwin.tar.gz" => assert_eq!(
                    hash,
                    "2eb106e7eda2b2c8300a19eebbe8258ece5624305a2e6248da98cfbb9cccbd62"
                ),
                "ImageMagick-i386-pc-solaris2.11.tar.gz" => assert_eq!(
                    hash,
                    "ed3ec2340dd84c7b4015fcd773ac32ab80b5c268aff234225c23ba7a6a98f326"
                ),
                _ => unreachable!(),
            }
        }
    }
}
