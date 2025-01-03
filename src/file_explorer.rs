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
    // Container for all explored files matching given dependency
    let mut files;

    if args.threaded {
        files = parallel_explorer::collect_files(&args.path, args.deep)?;
    } else {
        files = hashbrown::HashMap::new();
        explorer::collect_files(&args.path, &mut files, args.deep)?;
    }

    Ok(files)
}

fn filter_entries(
    path: &PathBuf,
    deep: bool,
) -> Result<impl Iterator<Item = DirEntry> + '_, error::MoleError> {
    fn is_valid_entry(entry: &DirEntry, deep: bool) -> bool {
        let path = entry.path();
        path.is_dir()
            || path
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| ext == ETOML || (deep && ext == ELOCK))
    }

    let entries = fs::read_dir(path)?.filter_map(move |entry| match entry {
        Ok(entry) => {
            if is_valid_entry(&entry, deep) {
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
