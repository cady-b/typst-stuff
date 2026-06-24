use std::path::Path;

use crate::cache::{Cache, TypstVersion};

/// Requires a `.canonicalize()`ed path!
pub fn version(binary: impl AsRef<Path>, cache: &Cache) -> Option<TypstVersion> {
    cache
        .iter()
        .find(|(b, _)| b == binary.as_ref())
        .map(|(_, v)| v.to_owned())
}

pub fn binary(version: &TypstVersion, cache: &Cache) -> Option<String> {
    cache
        .iter()
        .find(|(_, v)| v == version)
        .map(|(b, _)| b.to_owned())
}
