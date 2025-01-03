use super::filter_entries;
use crate::{
    error::MoleError,
    file_explorer::{CargoFiles, CLOCK, CTOML},
};
use hashbrown::HashMap;
use rayon::{prelude::*, ThreadPoolBuilder};
use std::{path::PathBuf, sync::mpsc::Sender};

pub fn collect_files(
    path: &PathBuf,
    deep: bool,
    symlinks: bool,
) -> Result<HashMap<String, CargoFiles>, MoleError> {
    let mut files: HashMap<String, CargoFiles> = hashbrown::HashMap::new();

    let (sender, receiver) = std::sync::mpsc::channel::<(String, PathBuf)>();

    let pool = ThreadPoolBuilder::new().num_threads(4).build()?;
    pool.install(|| -> Result<(), MoleError> {
        explore(path, deep, sender, symlinks)?;
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
    symlinks: bool,
) -> Result<(), MoleError> {
    let entries = filter_entries(path, deep, symlinks);

    match entries {
        Err(e) => {
            eprintln!("Error accessing entry: {} | {}", path.display(), e);
        }
        Ok(entries) => {
            entries
                .collect::<Vec<_>>()
                .into_par_iter()
                .with_max_len(1)
                .for_each(|entry| {
                    if let Err(e) = (|| -> Result<(), MoleError> {
                        let path = entry.path();

                        if path.is_dir() {
                            explore(&path, deep, sender.clone(), symlinks)?;
                        } else if path.is_file() {
                            if let (Some(parent), Some(file_name)) = (
                                path.parent().map(|p| p.to_string_lossy()),
                                path.file_name().and_then(|f| f.to_str()),
                            ) {
                                match file_name {
                                    CTOML => {
                                        sender.send((parent.to_string(), path.clone()))?;
                                    }
                                    CLOCK if deep => {
                                        sender.send((parent.to_string(), path.clone()))?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Ok(())
                    })() {
                        eprintln!("Error processing entry: {} {}", path.display(), e);
                    }
                });
        }
    }

    Ok(())
}
