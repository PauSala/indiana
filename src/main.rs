pub mod cli;
pub mod error;
pub mod file_explorer;
pub mod parallel_explorer;
pub mod parser;
pub mod printer;

use clap::Parser;
use cli::Args;
use file_explorer::{CargoFiles, CLOCK, CTOML};
use hashbrown::HashMap;
use parser::data::OutputRow;
use printer::pretty_table::print_table;
use std::{path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let args = Args::parse();

    match explore(args) {
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
        Ok(deps) => {
            let parsed_deps = deps
                .into_iter()
                .map(|package| {
                    let res: [String; 3] =
                        [package.package_name, package.dep_version, package.path];
                    res
                })
                .collect();
            print_table(vec!["PACKAGE", "VERSION", "PATH"], parsed_deps);
            ExitCode::SUCCESS
        }
    }
}

fn explore(args: Args) -> Result<Vec<OutputRow>, error::MoleError> {
    // Container for all explored files matching given dependency
    let mut files: HashMap<String, CargoFiles> = hashbrown::HashMap::new();
    // file_explorer::collect_files(&args.path, &mut files, args.deep)?;

    let (sender, reciever) = std::sync::mpsc::channel::<(String, PathBuf)>();

    parallel_explorer::collect_files(&args.path, args.deep, sender)?;
    for (key, value) in reciever {
        match value.file_name().and_then(|f| f.to_str()) {
            Some(CTOML) => {
                files.entry(key).or_default().ctoml = Some(value.clone());
            }
            Some(CLOCK) => {
                files.entry(key).or_default().clock = Some(value.clone());
            }
            _ => {}
        }
    }
    parser::FileParser::new().parse(files, &args.name)
}
