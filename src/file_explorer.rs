use crate::error::MoleError;
use hashbrown::HashMap;
use std::{fs, path::PathBuf};

pub const CTOML: &str = "Cargo.toml";
pub const CLOCK: &str = "Cargo.lock";

#[derive(Default)]
pub struct CargoFiles {
    pub ctoml: Option<PathBuf>,
    pub clock: Option<PathBuf>,
}

/// Collects all cargo files in a given directory and its subdirectories.
///
/// # Arguments
///
/// * `path` - The path to the directory to search in.
/// * `target` - The hashmap to store the found files in.
/// * `deep` - A flag to indicate whether to include Cargo.lock as well.
pub fn collect_files(
    path: &PathBuf,
    target: &mut HashMap<String, CargoFiles>,
    deep: bool,
) -> Result<(), MoleError> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string());

    if let Ok(entries) = entries {
        for entry in entries {
            let entry = entry.map_err(MoleError::IoError)?;
            let path = entry.path();

            if path.is_dir() {
                collect_files(&path, target, deep)?;
            } else if path.is_file() {
                if let (Some(parent), Some(file_name)) = (
                    path.parent().and_then(|p| p.to_str().map(String::from)),
                    path.file_name().and_then(|f| f.to_str()),
                ) {
                    match file_name {
                        CTOML => {
                            target.entry(parent).or_default().ctoml = Some(path.clone());
                        }
                        CLOCK if deep => {
                            target.entry(parent).or_default().clock = Some(path.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    } else {
        // Do not fail if any file cannot be read for any reason.
        // Just print the file in the standart error.
        eprintln!("Cannot access: {:?}", path);
    }
    Ok(())
}
