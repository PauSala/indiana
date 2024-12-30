use clap::Parser;
use file_utils::collect_files;
use parser::process_packages;
use prettytable::print_table;
use std::{collections::HashMap, path::PathBuf, time::Instant};

pub mod file_utils;
pub mod package;
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
    let start = Instant::now();
    collect_files(&path, &mut files, args.deep)?;
    let time = start.elapsed();
    println!("Elapsed: {:?}", time);

    let start = Instant::now();
    let found = process_packages(files, name)?;
    let time = start.elapsed();
    println!("Elapsed: {:?}", time);

    print_table(
        vec![
            "PACKAGE",
            &format!("{} VERSION", args.name.to_ascii_uppercase()),
            "PATH",
        ],
        found,
    );

    Ok(())
}
