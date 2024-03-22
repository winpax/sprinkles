use std::collections::HashMap;

use regex::Regex;
use strum::{Display, EnumIter};

use crate::ops::Substitute;

#[derive(Debug, thiserror::Error)]
pub enum TextError {
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Base64 decoding: {0}")]
    Base64Error(#[from] base64::DecodeError),
}

// Convert from https://github.com/ScoopInstaller/Scoop/blob/f93028001fbe5c78cc41f59e3814d2ac8e595724/lib/autoupdate.ps1#L75

#[derive(Debug, Copy, Clone, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
enum RegexTemplates {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Checksum,
    Base64,
}

impl From<RegexTemplates> for &'static str {
    fn from(value: RegexTemplates) -> Self {
        match value {
            RegexTemplates::Md5 => r"([a-fA-F0-9]{32})",
            RegexTemplates::Sha1 => r"([a-fA-F0-9]{40})",
            RegexTemplates::Sha256 => r"([a-fA-F0-9]{64})",
            RegexTemplates::Sha512 => r"([a-fA-F0-9]{128})",
            RegexTemplates::Checksum => r"([a-fA-F0-9]{32,128})",
            RegexTemplates::Base64 => r"([a-zA-Z0-9+\/=]{24,88})",
        }
    }
}

impl RegexTemplates {
    fn into_substitute_map() -> HashMap<String, String> {
        use strum::IntoEnumIterator;

        let mut map = HashMap::new();

        for field in Self::iter() {
            let regex: &'static str = field.into();

            map.insert(field.to_string(), regex.to_string());
        }

        map
    }
}

pub fn parse_text(
    source: impl AsRef<str>,
    substitutions: HashMap<String, String>,
    regex: String,
) -> Result<Option<String>, TextError> {
    // TODO: Incorporate file_names

    let regex = if regex.is_empty() {
        r"^\s*([a-fA-F0-9]+)\s*$".to_string()
    } else {
        regex
    };

    let substituted = {
        let mut regex = regex;

        // Substitute regex templates for finding hashes
        regex.substitute(&RegexTemplates::into_substitute_map(), false);
        // Substitute provided substitutions (i.e url, basename, etc.)
        regex.substitute(&substitutions, true);

        debug!("{regex}");

        Regex::new(&regex)?
    };

    let mut hash = substituted
        .find(source.as_ref())
        .map(|hash| hash.as_str().replace(' ', ""));

    // Convert base64 encoded hashes
    if let Some(hash) = hash.as_mut() {
        let base64_regex = Regex::new(
            r"^(?:[A-Za-z0-9+\/]{4})*(?:[A-Za-z0-9+\/]{2}==|[A-Za-z0-9+\/]{3}=|[A-Za-z0-9+\/]{4})$",
        )
        .expect("valid base64 regex");

        if base64_regex.is_match(hash) {
            let invalid_base64 =
                Regex::new(r"^[a-fA-F0-9]+$").expect("valid \"invalid base64\" regex");

            // Detects an invalid base64 string
            if !invalid_base64.is_match(hash) || [32, 40, 64, 128].contains(&hash.len()) {
                use base64::prelude::*;

                let decoded_hash = if let Ok(decoded) = BASE64_STANDARD.decode(hash.as_bytes()) {
                    let mut decoded_hash = String::new();

                    decoded
                        .into_iter()
                        .for_each(|byte| decoded_hash += &format!("{byte:x}"));

                    decoded_hash
                } else {
                    hash.clone()
                };

                *hash = decoded_hash;
            }
        }
    }

    if hash.is_none() {
        let filename_regex = {
            let regex = r"([a-fA-F0-9]{32,128})[\x20\t]+.*`$basename(?:[\x20\t]+\d+)?"
                .to_string()
                .into_substituted(&substitutions, true);

            Regex::new(&regex)?
        };

        hash = filename_regex
            .find(source.as_ref())
            .map(|hash| hash.as_str().to_string());

        if hash.is_none() {
            let metalink_regex = Regex::new(r"<hash[^>]+>([a-fA-F0-9]{64})")?;

            hash = metalink_regex
                .find(source.as_ref())
                .map(|hash| hash.as_str().to_string());
        }
    }

    hash = hash.map(|hash| hash.to_lowercase());

    Ok(hash)
}
