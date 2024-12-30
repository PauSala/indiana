use std::{collections::HashMap, fs, path::PathBuf};

use crate::parser::PackageFiles;

const TOML: &str = "Cargo.toml";
const LOCK: &str = "Cargo.lock";

pub fn collect_files(
    path: &PathBuf,
    target: &mut HashMap<String, PackageFiles>,
    deep: bool,
) -> Result<(), String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string());
    if let Ok(entries) = entries {
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, target, deep)?;
            } else if path.is_file() {
                if let Some(parent) = path.parent() {
                    let parent_str = parent.to_str().unwrap().to_string();
                    let file_name = path.file_name().unwrap().to_str().unwrap();

                    match file_name {
                        TOML => {
                            target.entry(parent_str).or_default().ctoml = Some(path.clone());
                        }
                        LOCK if deep => {
                            target.entry(parent_str).or_default().clock = Some(path.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    } else {
        eprintln!("Cannot access: {:?}", path);
    }

    Ok(())
}
