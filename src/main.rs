use clap::Parser;
use mole::{
    cli::Args,
    error, file_explorer,
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
    parser::FileParser::new().parse(file_explorer::explore(&args)?, &args.name)
}
