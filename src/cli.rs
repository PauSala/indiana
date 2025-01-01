use std::path::PathBuf;

use clap::Parser;

/// Searches recursively for a specified cargo dependency in all projects within a given directory.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The name of the dependency to search for.
    pub name: String,

    /// The directory to search in. Defaults to the current directory.
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Flag to indicate whether to search for the dependency in Cargo.lock as well.
    #[arg(short, long, default_value_t = false)]
    pub deep: bool,

    /// Flag to indicate whether to explore files in parallel.
    #[arg(short, long, default_value_t = false)]
    pub threaded: bool,

    /// Semver filter to filter the dependency by.
    /// Accepts a single semver version or a range in quotes, coma separated.
    ///
    /// Example: ">= 1.0.0, <2.0.0"
    #[arg(short, long, default_value = None)]
    pub filter: Option<String>,
}
