use clap::Parser;

/// Searches for a specified cargo dependency in all projects within a given directory.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The name of the dependency to search for.
    #[arg(short, long)]
    pub name: String,

    /// The directory to search in. Defaults to the current directory.
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Flag to indicate whether to search for the dependency in Cargo.lock as well.
    #[arg(short, long, default_value_t = false)]
    pub deep: bool,
}
