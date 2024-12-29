use clap::Parser;
use commands::Args;
use file_utils::collect_files;
use parser::process_packages;
use prettytable::print_table;
use std::{collections::HashMap, path::PathBuf};

pub mod commands;
pub mod file_utils;
pub mod package;
pub mod parser;
pub mod prettytable;

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

    let packages: HashMap<String, Vec<PathBuf>> =
        files.into_iter().fold(HashMap::new(), |mut acc, file| {
            if let Some(parent) = file.parent() {
                acc.entry(parent.to_str().unwrap().to_string())
                    .or_default()
                    .push(file);
            }
            acc
        });

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
