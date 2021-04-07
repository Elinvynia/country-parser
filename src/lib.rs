//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # country-parser
//!
//! A simple country parser library. Currently contains all ISO 3166-1 countries.
//!
//!
//! [ci]: https://github.com/Elinvynia/country-parser/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/country-parser/Rust/master?style=flat-square
//! [docs]: https://docs.rs/country-parser
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/country-parser
//! [crate-version]: https://img.shields.io/crates/v/country-parser.svg?style=flat-square

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

const DATA: &str = include_str!("data.ron");

/// Represents a country and holds various information about it.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    /// The "official" name of the country, or at least as close as it gets.
    pub official_name: String,
    /// The commonly used short form of the country name.
    pub short_name: String,
    /// ISO 3166-1 alpha-2 code of the country.
    pub iso2: String,
    /// ISO 3166-1 alpha-3 code of the country.
    pub iso3: String,
    /// ISO 3166-1 numeric-3 code of the country.
    pub num3: u16,
}

/// Attempts to parse the input into a valid country, checking all possible fields.
pub fn parse<T: AsRef<str>>(info: T) -> Option<Country> {
    let info = info.as_ref().to_lowercase();

    // Guaranteed to not panic.
    let countries: Vec<Country> = ron::from_str(DATA).unwrap();

    countries.into_iter().find(|c| {
        info == c.official_name.to_lowercase()
            || info == c.short_name.to_lowercase()
            || info == c.iso2.to_lowercase()
            || info == c.iso3.to_lowercase()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let _: Vec<Country> = ron::from_str(DATA).unwrap();
    }

    #[test]
    fn test_country() {
        assert_eq!(parse("andorra").unwrap().iso2, "AN".to_string());
    }
}
