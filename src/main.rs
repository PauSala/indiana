use clap::Parser;
use file_utils::collect_files;
use parser::process_packages;
use prettytable::print_table;
use std::{collections::HashMap, path::PathBuf};

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
    let mut dep_files = vec!["Cargo.toml"];
    if args.deep {
        dep_files.push("Cargo.lock");
    }

    let mut files = Vec::new();
    collect_files(&path, &mut files, &dep_files)?;

    let mut packages: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in files {
        if let Some(parent) = file.parent() {
            packages
                .entry(parent.to_str().unwrap().to_string())
                .or_insert_with(Vec::new)
                .push(file);
        }
    }

    let found = process_packages(packages, name)?;

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
