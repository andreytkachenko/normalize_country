use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader},
};

use normalize_country::CountryNameNormalizer;

fn main() {
    let cn = CountryNameNormalizer::new("./countries/en.toml").unwrap();
    let file = std::fs::File::open("./examples/countries.csv").unwrap();
    let mut map = BTreeMap::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap().to_lowercase();

        if line != "xa" && line != "xy" {
            *map.entry(cn.normalize_country(&line).unwrap().alpha2())
                .or_insert(0) += 1;
        }
    }

    println!("{:#?}", map);
}
