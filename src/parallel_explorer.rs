use crate::{error::MoleError, file_explorer::CargoFiles};
use std::{fs, path::PathBuf, sync::mpsc::Sender};

pub const CTOML: &str = "Cargo.toml";
pub const CLOCK: &str = "Cargo.lock";

use hashbrown::HashMap;
use rayon::prelude::*;

pub fn collect_files(path: &PathBuf, deep: bool) -> Result<HashMap<String, CargoFiles>, MoleError> {
    let mut files: HashMap<String, CargoFiles> = hashbrown::HashMap::new();

    let (sender, reciever) = std::sync::mpsc::channel::<(String, PathBuf)>();

    explore(path, deep, sender)?;
    for (key, value) in reciever {
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

/// Collects all cargo files in a given directory and its subdirectories.
///
/// # Arguments
///
/// * `path` - The path to the directory to search in.
/// * `target` - The hashmap to store the found files in.
/// * `deep` - A flag to indicate whether to include Cargo.lock as well.
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
                            CTOML | CLOCK => {
                                sender.send((parent, path.clone()))?;
                            }
                            _ => {}
                        }
                    }
                }
                Ok(())
            }) {
                // Do not fail if any file cannot be read for any reason.
                // Just print the file in the standart error.
                eprintln!("Error processing entry: {} {}", path.display(), e);
            }
        });

    Ok(())
}
