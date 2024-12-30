use clap::Parser;
use file_utils::collect_files;
use hashbrown::HashMap;
use parser::FileParser;
use prettytable::print_table;
use std::path::PathBuf;

pub mod file_utils;
pub mod parser;
pub mod prettytable;

/// Searches for a specified cargo dependency in all projects within a given directory.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the dependency to search for.
    #[arg(short, long)]
    name: String,

    /// The directory to search in. Defaults to the current directory.
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Flag to indicate whether to search for the dependency in Cargo.lock as well.
    #[arg(short, long, default_value_t = false)]
    deep: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let path = PathBuf::from(args.path);
    let name = &args.name;

    let mut files = HashMap::new();
    collect_files(&path, &mut files, args.deep)?;

    let parsed_deps = FileParser::new().parse(files, name)?;

    print_table(
        vec![
            "PACKAGE",
            &format!("{} VERSION", args.name.to_ascii_uppercase()),
            "PATH",
        ],
        parsed_deps,
    );

    Ok(())
}
