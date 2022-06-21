# Country name normalization library

Convert country names and codes to a standard.

## Usage example
```rust 
let cn = CountryNameNormalizer::new("./countries/en.toml").unwrap();
let st_kittis_nevis = cn.normalize_country("St. Kitts & Nevis").unwrap();

println!("{:?}", st_kittis_nevis)

//
// Country {
//   aliases: Some(["Federation of Saint Christopher and Nevi"]),
//   alpha2: "KN",
//   alpha3: "KNA",
//   fifa: "SKN",
//   ioc: "SKN",
//   iso_name: "Saint Kitts And Nevis",
//   numeric: 659,
//   official: "Federation of Saint Kitts and Nevis",
//   short: "Saint Kitts And Nevis",
//   emoji: "🇰🇳",
//   shortcode: ":flag-kn:",
// }
```