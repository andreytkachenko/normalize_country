pub use error::Error;
use lazy_static::lazy_static;
use lazy_static_include::lazy_static_include_bytes;
use regex::Regex;
use serde::{Deserialize, Serialize};
use smallstr::SmallString;
use std::collections::HashMap;

mod error;

lazy_static! {
    static ref PARSER: Regex = Regex::new(r"\s*([^~!?,&(){}\[\]\s]+|[~!?,&(){}\[\]])").unwrap();
}

lazy_static_include_bytes! {
    ENG_DATA => "countries/en.toml",
}

#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct CountryRef<'a> {
    pub aliases: Option<Vec<&'a str>>,
    pub alpha2: &'a str,
    pub alpha3: &'a str,
    pub fifa: &'a str,
    pub ioc: &'a str,
    pub iso_name: &'a str,
    pub numeric: i32,
    pub official: &'a str,
    pub short: &'a str,
    pub emoji: &'a str,
    pub shortcode: &'a str,
}

impl<'a> CountryRef<'a> {
    pub fn to_owned(&self) -> Country {
        let mut alpha2 = self.alpha2.as_bytes().iter().copied();
        let mut alpha3 = self.alpha3.as_bytes().iter().copied();
        let mut fifa = self.fifa.as_bytes().iter().copied();
        let mut ioc = self.ioc.as_bytes().iter().copied();

        Country {
            aliases: self
                .aliases
                .as_ref()
                .map(|x| x.iter().map(|&x| x.into()).collect()),

            alpha2: [alpha2.next().unwrap_or(b' '), alpha2.next().unwrap_or(b' ')],
            alpha3: [
                alpha3.next().unwrap_or(b' '),
                alpha3.next().unwrap_or(b' '),
                alpha3.next().unwrap_or(b' '),
            ],
            fifa: [
                fifa.next().unwrap_or(b' '),
                fifa.next().unwrap_or(b' '),
                fifa.next().unwrap_or(b' '),
            ],
            ioc: [
                ioc.next().unwrap_or(b' '),
                ioc.next().unwrap_or(b' '),
                ioc.next().unwrap_or(b' '),
            ],
            iso_name: self.iso_name.into(),
            numeric: self.numeric,
            official: self.official.into(),
            short: self.short.into(),
            emoji: self.emoji.chars().next().unwrap_or(' '),
            shortcode: self.shortcode.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub aliases: Option<Vec<SmallString<[u8; 23]>>>,
    pub alpha2: [u8; 2],
    pub alpha3: [u8; 3],
    pub fifa: [u8; 3],
    pub ioc: [u8; 3],
    pub iso_name: SmallString<[u8; 23]>,
    pub numeric: i32,
    pub official: SmallString<[u8; 23]>,
    pub short: SmallString<[u8; 23]>,
    pub emoji: char,
    pub shortcode: SmallString<[u8; 23]>,
}

impl Country {
    /// Get the country's alpha2.
    #[must_use]
    #[inline]
    pub fn alpha2(&self) -> &str {
        std::str::from_utf8(&self.alpha2).unwrap()
    }

    /// Get the country's alpha3.
    #[must_use]
    #[inline]
    pub fn alpha3(&self) -> &str {
        std::str::from_utf8(&self.alpha3).unwrap()
    }

    /// Get the country's fifa.
    #[must_use]
    #[inline]
    pub fn fifa(&self) -> &str {
        std::str::from_utf8(&self.fifa).unwrap()
    }

    /// Get the country's ioc.
    #[must_use]
    #[inline]
    pub fn ioc(&self) -> &str {
        std::str::from_utf8(&self.ioc).unwrap()
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.numeric == other.numeric
    }
}

impl Eq for Country {}

impl PartialOrd for Country {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.alpha3.partial_cmp(&other.alpha3)
    }
}

impl Ord for Country {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.alpha3.cmp(&other.alpha3)
    }
}

pub struct CountryNameNormalizer {
    countries: Vec<Country>,
    keys: HashMap<String, usize>,
}

impl Default for CountryNameNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl CountryNameNormalizer {
    pub fn new() -> Self {
        let data: HashMap<&str, CountryRef> = toml::from_slice(&ENG_DATA).unwrap();

        let mut countries = Vec::with_capacity(data.len());
        let mut keys = HashMap::new();

        for (_, value) in data.into_iter() {
            let idx = countries.len();
            countries.push(value.to_owned());

            Self::add(&mut keys, value.alpha2, idx);
            Self::add(&mut keys, value.alpha3, idx);
            Self::add(&mut keys, value.fifa, idx);
            Self::add(&mut keys, value.ioc, idx);
            Self::add(&mut keys, value.iso_name, idx);
            Self::add(&mut keys, value.official, idx);
            Self::add(&mut keys, value.short, idx);
            // Self::add(&mut keys, value.emoji.as_str(), idx);

            if let Some(aliases) = &value.aliases {
                for alias in aliases {
                    Self::add(&mut keys, alias, idx);
                }
            }
        }

        Self { countries, keys }
    }

    pub fn normalize_country(&self, name: &str) -> Option<&Country> {
        let key = normalize_name(name);
        let index = self.keys.get(&key)?;
        self.countries.get(*index)
    }

    fn add(keys: &mut HashMap<String, usize>, w: &str, idx: usize) {
        let s = normalize_name(w);
        if !s.is_empty() {
            keys.insert(s, idx);
        }
    }
}

fn normalize_name(name: &str) -> String {
    let name = name.to_lowercase();

    let iter = PARSER
        .captures_iter(&name)
        .map(|cap| cap.get(1).unwrap().as_str());

    let mut res = String::new();

    for w in iter {
        let w = w.trim();

        if w.is_empty() {
            continue;
        } else if w.len() == 1 {
            match w.chars().next().unwrap() {
                '&' => res.push_str("and "),
                p if p.is_ascii_punctuation() => (),
                ch => {
                    res.push(ch);
                    res.push(' ');
                }
            }
        } else {
            match w {
                "islas" | "minor" | "the" => {}
                "st." => res.push_str("saint "),
                "u.s." => {
                    res.push_str("united states ");
                }
                "u.s.a." => {
                    res.push_str("united states of america ");
                }
                x => {
                    res.push_str(x);
                    res.push(' ');
                }
            }
        }
    }

    res.pop();
    res
}
