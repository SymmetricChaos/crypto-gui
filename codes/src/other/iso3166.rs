use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use lazy_static::lazy_static;

const COUNTRY_CODES_STR: &'static str = include_str!("country_codes.txt");

lazy_static! {
    pub static ref COUNTRY_CODES: Vec<Vec<String>> = {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::Fields)
            .from_reader(COUNTRY_CODES_STR.as_bytes());

        let mut out = Vec::with_capacity(249);

        for record in rdr.records() {
            let fields = record
                .into_iter()
                .map(|r| r.as_slice().to_string())
                .collect_vec();
            out.push(fields)
        }
        out
    };
}

pub enum Iso3166Version {
    Alpha2,
    Alpha3,
    Numeric,
}

pub struct Iso3166 {
    version: Iso3166Version,
}

impl Default for Iso3166 {
    fn default() -> Self {
        Iso3166 {
            version: Iso3166Version::Alpha2,
        }
    }
}

impl Iso3166 {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&char, String)> + '_> {
        todo!()
    }
}

impl Code for Iso3166 {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod iso3166_tests {

    use super::*;
    use csv;

    #[test]
    fn csv_manip() {
        // Build the CSV reader and iterate over each record.
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::Fields)
            .from_path("src\\other\\country_codes.txt")
            .unwrap();
        // let mut rdr = csv::Reader::from_path().unwrap();

        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            match result {
                Ok(record) => println!("{:?}", record),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
