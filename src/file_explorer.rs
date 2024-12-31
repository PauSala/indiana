mod explorer;
mod parallel_explorer;

use std::path::PathBuf;

use hashbrown::HashMap;

use crate::{cli::Args, error};

pub const CTOML: &str = "Cargo.toml";
pub const CLOCK: &str = "Cargo.lock";

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
