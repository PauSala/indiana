use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoleError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}
