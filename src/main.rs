use clap::Parser;
use file_utils::{collect_files, filter_by_extension};
use package::Package;
use prettytable::print_table;
use std::{collections::HashMap, fs, path::PathBuf};

pub mod file_utils;
pub mod package;
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
    let mut found = Vec::new();

    let mut files = Vec::new();
    let mut dep_files = vec!["Cargo.toml"];

    if args.deep {
        dep_files.push("Cargo.lock");
    }

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

    for (_, value) in packages {
        if let Some(toml) = filter_by_extension(&value, "toml") {
            // Parse .toml
            let toml_file = fs::read_to_string(&toml).map_err(|e| e.to_string())?;
            let parsed = Package::parse_toml(
                &toml_file,
                &args.name,
                toml.to_str().expect("Path must be a file"),
            );

            let package_name;
            if let Some(parsed) = parsed {
                package_name = parsed.name.clone();
                found.push(parsed);
            } else {
                package_name = "Package name not found".to_string();
            }

            // parse .lock
            if let Some(lock) = filter_by_extension(&value, "lock") {
                let lock_file = fs::read_to_string(&lock).map_err(|e| e.to_string())?;
                let parsed = Package::parse_lock(
                    &lock_file,
                    &args.name,
                    lock.to_str().unwrap(),
                    package_name,
                );
                found.extend(parsed);
            }
        }
    }

    found.sort_by(|a, b| a.path.cmp(&b.path));

    let collected: Vec<Vec<String>> = found
        .into_iter()
        .map(|package| {
            let mut res = Vec::new();
            res.push(package.name);
            res.push(package.dep_version);
            res.push(package.path);
            res
        })
        .collect();

    print_table(
        vec![
            "PACKAGE",
            &format!("{} VERSION", args.name.to_ascii_uppercase()),
            "PATH",
        ],
        collected,
    );

    Ok(())
}
