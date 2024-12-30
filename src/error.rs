use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoleError {
    #[error("An error occurred while parsing arguments: {0}")]
    ArgParseError(#[from] clap::Error),

    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}
