use hashbrown::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct DepInfo {
    pub version: String,
    pub path: String,
}

pub fn print_json(rows: HashMap<String, Vec<DepInfo>>) {
    match serde_json::to_string_pretty(&rows) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}
