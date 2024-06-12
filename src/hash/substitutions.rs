use std::collections::HashMap;

use derive_more::{Deref, DerefMut};
use url::Url;

use crate::{
    hash::url_ext::UrlExt,
    packages::models::manifest::{AliasArray, Installer, TOrArrayOfTs},
    version::Version,
};

fn replace_in_place(string: &mut String, from: &str, to: &str) {
    let current_string = string.clone();
    for (start, part) in current_string.match_indices(from) {
        string.replace_range(start..start + part.len(), to);
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct SubstitutionMap(HashMap<String, String>);

impl SubstitutionMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from_all(version: &Version, url: &Url) -> Self {
        let mut map = Self::new();

        map.append_version(version);
        map.append_url(url);

        map
    }

    pub fn substitute(&self, string: &mut String, regex_escape: bool) {
        SubstituteBuilder::String(string).substitute(self, regex_escape);
    }

    /// Append version information to the map
    pub fn append_version(&mut self, version: &Version) {
        self.extend(version.submap().0);
    }

    pub fn append_url(&mut self, url: &Url) {
        self.extend(url.submap().0);
    }
}

impl Default for SubstitutionMap {
    fn default() -> Self {
        Self::new()
    }
}

pub enum SubstituteBuilder<'a> {
    String(&'a mut String),
}

impl<'a> SubstituteBuilder<'a> {
    pub fn substitute(self, params: &SubstitutionMap, regex_escape: bool) {
        match self {
            SubstituteBuilder::String(new_entity) => {
                for key in params.keys() {
                    let value = params.get(key).unwrap();

                    if regex_escape {
                        replace_in_place(new_entity, key, &regex::escape(value));
                    } else {
                        replace_in_place(new_entity, key, value);
                    }
                }
            }
        }
    }
}

pub trait Substitute {
    fn substitute(&mut self, params: &SubstitutionMap, regex_escape: bool);

    #[must_use]
    fn into_substituted(mut self, params: &SubstitutionMap, regex_escape: bool) -> Self
    where
        Self: Clone,
    {
        self.substitute(params, regex_escape);
        self
    }
}

impl Substitute for String {
    fn substitute(&mut self, params: &SubstitutionMap, regex_escape: bool) {
        SubstituteBuilder::String(self).substitute(params, regex_escape)
    }
}

impl<T: Substitute> Substitute for TOrArrayOfTs<T> {
    fn substitute(&mut self, params: &SubstitutionMap, regex_escape: bool) {
        match self {
            TOrArrayOfTs::Single(s) => s.substitute(params, regex_escape),
            TOrArrayOfTs::Array(a) => {
                for s in a.iter_mut() {
                    s.substitute(params, regex_escape);
                }
            }
        }
    }
}

impl<T: Substitute> Substitute for Vec<T> {
    fn substitute(&mut self, params: &SubstitutionMap, regex_escape: bool) {
        self.iter_mut()
            .for_each(|s| s.substitute(params, regex_escape));
    }
}

impl<T: Substitute> Substitute for AliasArray<T> {
    fn substitute(&mut self, params: &SubstitutionMap, regex_escape: bool) {
        match self {
            AliasArray::NestedArray(TOrArrayOfTs::Single(s)) => s.substitute(params, regex_escape),
            AliasArray::NestedArray(TOrArrayOfTs::Array(s)) => s
                .iter_mut()
                .for_each(|s| s.substitute(params, regex_escape)),
            AliasArray::AliasArray(s) => s
                .iter_mut()
                .for_each(|s| s.substitute(params, regex_escape)),
        }
    }
}

impl Substitute for Installer {
    fn substitute(&mut self, params: &SubstitutionMap, regex_escape: bool) {
        self.file
            .as_mut()
            .map(|s| s.substitute(params, regex_escape));

        self.comment
            .as_mut()
            .map(|s| s.substitute(params, regex_escape));

        self.args
            .as_mut()
            .map(|s| s.substitute(params, regex_escape));

        self.script
            .as_mut()
            .map(|s| s.substitute(params, regex_escape));
    }
}

#[cfg(test)]
mod tests {
    use super::replace_in_place;

    #[test]
    fn test_replace_in_place() {
        let mut string = String::from("Hello, world!");
        let should_be = string.replace("world", "rust");

        replace_in_place(&mut string, "world", "rust");

        assert_eq!(string, should_be);
    }
}
