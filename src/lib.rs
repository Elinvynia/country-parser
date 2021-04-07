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

#[macro_use]
extern crate lazy_static;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// Attempts to parse the input into a valid country, checking all possible fields.
pub fn parse<T: AsRef<str>>(info: T) -> Option<Country> {
    let info = info.as_ref().to_lowercase();
    let countries: &Vec<Country> = &*DATA;

    countries
        .iter()
        .find(|c| {
            info == c.official_name.to_lowercase()
                || info == c.short_name.to_lowercase()
                || info == c.iso2.to_lowercase()
                || info == c.iso3.to_lowercase()
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country() {
        assert_eq!(parse("andorra").unwrap().iso2, "AN".to_string());
    }
}
