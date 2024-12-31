use crate::error::MoleError;
use std::{fs, path::PathBuf, sync::mpsc::Sender};

pub const CTOML: &str = "Cargo.toml";
pub const CLOCK: &str = "Cargo.lock";

use rayon::prelude::*;

/// Collects all cargo files in a given directory and its subdirectories.
///
/// # Arguments
///
/// * `path` - The path to the directory to search in.
/// * `target` - The hashmap to store the found files in.
/// * `deep` - A flag to indicate whether to include Cargo.lock as well.
pub fn collect_files(
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
                    collect_files(&path, deep, sender.clone())?;
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
                eprintln!("Error processing entry: {} {}", path.display(), e);
            }
        });

    Ok(())
}
