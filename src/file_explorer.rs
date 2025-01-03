mod explorer;
mod parallel_explorer;

use crate::{cli::Args, error};
use hashbrown::HashMap;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub const CTOML: &str = "Cargo.toml";
pub const ETOML: &str = "toml";

pub const CLOCK: &str = "Cargo.lock";
pub const ELOCK: &str = "lock";

#[derive(Default)]
pub struct CargoFiles {
    pub ctoml: Option<PathBuf>,
    pub clock: Option<PathBuf>,
}

pub fn explore(args: &Args) -> Result<HashMap<String, CargoFiles>, error::MoleError> {
    let mut files;

    if args.threaded {
        files = parallel_explorer::collect_files(&args.path, args.deep, args.symlinks)?;
    } else {
        files = hashbrown::HashMap::new();
        explorer::collect_files(&args.path, &mut files, args.deep, args.symlinks)?;
    }

    Ok(files)
}

fn filter_entries(
    path: &PathBuf,
    deep: bool,
    symlinks: bool,
) -> Result<impl Iterator<Item = DirEntry> + '_, error::MoleError> {
    let entries = fs::read_dir(path)?.filter_map(move |entry| match entry {
        Ok(entry) => {
            if symlinks {
                if filter_entry_with_symlinks(&entry, deep, symlinks) {
                    Some(entry)
                } else {
                    None
                }
            } else if filter_entry(&entry, deep) {
                Some(entry)
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!("Error accessing entry: {} | {e}", path.display());
            None
        }
    });

    Ok(entries)
}

fn filter_entry_with_symlinks(entry: &DirEntry, deep: bool, symlinks: bool) -> bool {
    let path = entry.path();
    path.is_dir() && (!path.is_symlink() || symlinks)
        || path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == ETOML || (deep && ext == ELOCK))
}

fn filter_entry(entry: &DirEntry, deep: bool) -> bool {
    let path = entry.path();
    path.is_dir()
        || path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == ETOML || (deep && ext == ELOCK))
}
