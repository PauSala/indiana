use clap::Parser;
use cli::Args;
use mole::{
    cli, error, file_explorer, parallel_explorer,
    parser::{self, data::OutputRow},
    printer::pretty_table::print_table,
};
use std::process::ExitCode;

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
    let mut files;

    if args.threaded {
        files = parallel_explorer::collect_files(&args.path, args.deep)?;
    } else {
        files = hashbrown::HashMap::new();
        file_explorer::collect_files(&args.path, &mut files, args.deep)?;
    }

    parser::FileParser::new().parse(files, &args.name)
}
