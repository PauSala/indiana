use crate::error::MoleError;
use hashbrown::HashMap;
use std::path::PathBuf;

use super::{filter_entries, CargoFiles, CLOCK, CTOML};

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
    let entries = filter_entries(path, deep);

    match entries {
        Err(e) => {
            eprintln!("Error accessing entry: {} | {}", path.display(), e);
        }
        Ok(entries) => {
            for entry in entries {
                let path = entry.path();

                if path.is_dir() {
                    collect_files(&path, target, deep)?;
                } else if path.is_file() {
                    if let (Some(parent), Some(file_name)) = (
                        path.parent().and_then(|p| Some(p.to_string_lossy())),
                        path.file_name().and_then(|f| f.to_str()),
                    ) {
                        match file_name {
                            CTOML => {
                                target.entry(parent.to_string()).or_default().ctoml =
                                    Some(path.clone());
                            }
                            CLOCK if deep => {
                                target.entry(parent.to_string()).or_default().clock =
                                    Some(path.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
