use std::str::FromStr;

use clap::ValueEnum;
use hashbrown::HashMap;
use json::{print_json, DepInfo};
use pretty_table::print_table;

use crate::parser::data::OutputRow;

pub mod json;
pub mod pretty_table;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PrintFormat {
    Table,
    Json,
}

impl FromStr for PrintFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(PrintFormat::Table),

            "json" => Ok(PrintFormat::Json),

            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

pub fn print(rows: Vec<OutputRow>, format: &PrintFormat) {
    match format {
        PrintFormat::Table => {
            let rows = rows
                .into_iter()
                .map(|package| [package.package_name, package.dep_version, package.path])
                .collect();
            print_table(vec!["PACKAGE", "VERSION", "PATH"], rows);
        }
        PrintFormat::Json => {
            let mut mapped: HashMap<String, Vec<DepInfo>> = HashMap::new();
            for row in rows {
                mapped.entry(row.package_name).or_default().push(DepInfo {
                    version: row.dep_version,
                    path: row.path,
                });
            }
            print_json(mapped);
        }
    }
}
