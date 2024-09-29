use quick_xml::{events::Event, name::QName, reader::Reader};

use super::super::{Error, Result};

pub fn find_rdf(input: impl AsRef<[u8]>, file_name: impl AsRef<str>) -> Result<String> {
    // This may well be "bad code"
    // It's probably not very battle resistant.
    // I'm honestly sick of dealing with XML
    // As such I'm leaving it for the time being
    // fuck xml :P

    let mut reader = Reader::from_reader(input.as_ref());
    reader.config_mut().trim_text(true);

    let mut buf = vec![];

    let mut about_tag = None;
    let mut hash_tag = None;

    let mut hash = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                if about_tag.is_some() {
                    if e.name() == QName(b"digest:sha256") {
                        assert!(
                            hash_tag.is_none(),
                            "woops something wasnt cleaned up properly"
                        );

                        std::mem::swap(&mut about_tag, &mut hash_tag);
                    }
                } else {
                    for attribute in e.attributes().flatten() {
                        let file_name = file_name.as_ref().to_string();

                        if attribute.key == QName(b"rdf:about")
                            && attribute.value.as_ref() == file_name.as_bytes()
                        {
                            about_tag = Some(file_name);
                        }
                    }
                }
            }

            Ok(Event::Text(e)) => {
                if hash_tag.is_some() {
                    hash = Some(e.unescape().unwrap().to_string());
                    break;
                }
            }

            _ => (),
        }
    }

    hash.ok_or(Error::NotFound)
}
