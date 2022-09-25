use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Country {
    official_name: String,
    short_name: String,
    iso2: String,
    iso3: String,
    num3: u16,
}

fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=src/data.ron");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");

    let in_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let in_path = Path::new(&in_dir).join("src").join("data.ron");
    let countries: Vec<Country> = ron::from_str(&fs::read_to_string(in_path).unwrap()).unwrap();

    let mut s = String::new();
    for country in countries {
        s.push_str(&format!("Country {{ official_name: String::from(\"{}\"), short_name: String::from(\"{}\"), iso2: String::from(\"{}\"), iso3: String::from(\"{}\"), num3: {} }}, ",
            country.official_name, country.short_name, country.iso2, country.iso3, country.num3));
    }

    fs::write(
        &dest_path,
        format!(
            "
use serde::{{Serialize, Deserialize}};

/// Represents a country and holds various information about it.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Country {{
    /// The 'official' name of the country, or at least as close as it gets.
    pub official_name: String,
    /// The commonly used short form of the country name.
    pub short_name: String,
    /// ISO 3166-1 alpha-2 code of the country.
    pub iso2: String,
    /// ISO 3166-1 alpha-3 code of the country.
    pub iso3: String,
    /// ISO 3166-1 numeric-3 code of the country.
    pub num3: u16,
}}

lazy_static! {{
    static ref DATA: Vec<Country> = vec![{}];
}}

",
            s
        ),
    )
    .unwrap();
}
