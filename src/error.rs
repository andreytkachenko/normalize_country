#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Data Parse Error: {0}")]
    DataParseError(#[from] toml::de::Error),

    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
}
