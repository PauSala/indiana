use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoleError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error("SendError: {0}")]
    SendError(#[from] std::sync::mpsc::SendError<(std::string::String, PathBuf)>),

    #[error("ThreadPoolBuildError: {0}")]
    ThreadPoolBuildError(#[from] rayon::ThreadPoolBuildError),

    #[error("SemverError: {0}")]
    SemverError(#[from] semver::Error),
}
