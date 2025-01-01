mod explorer;
mod parallel_explorer;

use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use hashbrown::HashMap;

use crate::{cli::Args, error};

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

fn read_entries<'a>(
    path: &'a PathBuf,
    deep: bool,
) -> Result<impl Iterator<Item = DirEntry> + 'a, error::MoleError> {
    let entries = fs::read_dir(path)?
        .inspect(|entry| {
            if let Err(ref e) = entry {
                eprintln!("Error accessing entry: {} | {e}", path.display());
            }
        })
        .filter_map(|entry| entry.ok())
        .filter(move |entry| {
            let path = entry.path();
            path.is_dir()
                || path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext == ETOML || (deep && ext == ELOCK))
        });
    Ok(entries)
}
