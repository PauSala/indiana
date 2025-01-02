use clap::Parser;
use mole::{
    cli::Args,
    error, file_explorer,
    parser::{self, data::OutputRow},
    printer::print,
};
use semver::VersionReq;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    let format = args.output.clone();

    match explore(args) {
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
        Ok(deps) => {
            print(deps, &format);
            ExitCode::SUCCESS
        }
    }
}

fn explore(args: Args) -> Result<Vec<OutputRow>, error::MoleError> {
    let filter = args.filter.as_deref().map(VersionReq::parse).transpose()?;

    Ok(mole::semver_filter::filter(
        filter,
        parser::FileParser::new().parse(file_explorer::explore(&args)?, &args.name)?,
    ))
}
