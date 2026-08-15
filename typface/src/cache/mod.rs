pub mod discover;
pub mod lookup;
pub mod read_cache;

pub use discover::rediscover_binaries;
pub use read_cache::Cache;
pub use read_cache::read_cache;

use std::hash::Hash;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum VersionPrefix {
    Raw,
    Versioned,
}

impl FromStr for VersionPrefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(Self::Raw),
            "v" => Ok(Self::Versioned),
            _ => Err(()),
        }
    }
}

impl Display for VersionPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Raw => "r",
            Self::Versioned => "v",
        })
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
// This struct was derived from: https://github.com/typst/typst/blob/c98e9103/crates/typst-utils/src/version.rs
// See the NOTICE file for more information
pub struct TypstVersion {
    /// Typst major version number.
    pub major: u32,
    /// Typst minor version number.
    pub minor: u32,
    /// Typst patch version number.
    pub patch: u32,
    /// Raw, unmodified version string.
    pub raw: Option<String>,
    /// The raw commit hash.
    pub commit: Option<String>,
}

impl PartialEq for TypstVersion {
    fn eq(&self, other: &Self) -> bool {
        if self.raw.is_none() && other.raw.is_none() {
            self.triplet() == other.triplet()
        } else {
            matches!((&self.raw, &other.raw), (Some(a), Some(b)) if a == b)
        }
    }
}

impl Hash for TypstVersion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.prefix() {
            VersionPrefix::Raw => {
                self.raw.hash(state);
            }
            VersionPrefix::Versioned => {
                self.triplet().hash(state);
            }
        }
    }
}

impl FromStr for TypstVersion {
    type Err = std::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match Self::parse_triplet(&mut unscanny::Scanner::new(value)) {
            Ok((major, minor, patch)) => Ok(Self::new(major, minor, patch)),
            Err(_) => Ok(Self::new_raw(value.to_owned())),
        }
    }
}

impl TypstVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            raw: None,
            commit: None,
        }
    }

    pub fn new_raw(v: String) -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 0,
            raw: Some(v),
            commit: None,
        }
    }

    pub fn prefix(&self) -> VersionPrefix {
        match self.raw {
            Some(_) => VersionPrefix::Raw,
            None => VersionPrefix::Versioned,
        }
    }

    pub fn triplet(&self) -> (u32, u32, u32) {
        (self.major, self.minor, self.patch)
    }

    pub fn stringify(&self) -> String {
        self.raw
            .to_owned()
            .unwrap_or_else(|| format!("{}.{}.{}", self.major, self.minor, self.patch))
    }

    pub fn parse_triplet(
        s: &mut unscanny::Scanner,
    ) -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
        let major = s.eat_while(char::is_ascii_digit).parse::<u32>()?;
        if !s.eat_if(".") {
            Err("missing dot after major version")?;
        }

        let minor = s.eat_while(char::is_ascii_digit).parse::<u32>()?;
        if !s.eat_if(".") {
            Err("missing dot after minor version")?;
        }

        let patch = s.eat_while(char::is_ascii_digit).parse::<u32>()?;

        Ok((major, minor, patch))
    }
}
