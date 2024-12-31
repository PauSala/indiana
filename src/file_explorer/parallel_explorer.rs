use crate::{
    error::MoleError,
    file_explorer::{CargoFiles, CLOCK, CTOML},
};
use std::{fs, path::PathBuf, sync::mpsc::Sender};

use hashbrown::HashMap;
use rayon::{prelude::*, ThreadPoolBuilder};

pub fn collect_files(path: &PathBuf, deep: bool) -> Result<HashMap<String, CargoFiles>, MoleError> {
    let mut files: HashMap<String, CargoFiles> = hashbrown::HashMap::new();

    let (sender, receiver) = std::sync::mpsc::channel::<(String, PathBuf)>();

    // Limit the number of threads used by rayon
    let pool = ThreadPoolBuilder::new().num_threads(4).build()?;
    pool.install(|| -> Result<(), MoleError> {
        explore(path, deep, sender)?;
        Ok(())
    })?;

    for (key, value) in receiver {
        match value.file_name().and_then(|f| f.to_str()) {
            Some(CTOML) => {
                files.entry(key).or_default().ctoml = Some(value.clone());
            }
            Some(CLOCK) => {
                files.entry(key).or_default().clock = Some(value.clone());
            }
            _ => {}
        }
    }
    Ok(files)
}

/// Recursively searches for Cargo.toml and Cargo.lock files in a given directory.
/// The found files are sent through the given sender.
///
/// # Arguments
///
/// * `path` - The path to the directory to search in.
/// * `deep` - A flag to indicate whether to include Cargo.lock as well.
/// * `sender` - The sender channel to send found files.
pub fn explore(
    path: &PathBuf,
    deep: bool,
    sender: Sender<(String, PathBuf)>,
) -> Result<(), MoleError> {
    let entries = fs::read_dir(path)?;

    entries
        .par_bridge()
        .map(|entry| entry.map_err(MoleError::IoError))
        .for_each(|entry_result| {
            if let Err(e) = entry_result.and_then(|entry| {
                let path = entry.path();

                if path.is_dir() {
                    explore(&path, deep, sender.clone())?;
                } else if path.is_file() {
                    if let (Some(parent), Some(file_name)) = (
                        path.parent()
                            .and_then(|p| p.canonicalize().ok())
                            .and_then(|p| p.to_str().map(String::from)),
                        path.file_name().and_then(|f| f.to_str()),
                    ) {
                        match file_name {
                            CTOML => {
                                sender.send((parent, path.clone()))?;
                            }
                            CLOCK if deep => {
                                sender.send((parent, path.clone()))?;
                            }
                            _ => {}
                        }
                    }
                }
                Ok(())
            }) {
                // Do not fail if any file cannot be read for any reason.
                // Just print the file in the standard error.
                eprintln!("Error processing entry: {} {}", path.display(), e);
            }
        });

    Ok(())
}
