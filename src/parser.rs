use std::{collections::HashMap, fs, path::PathBuf};

use crate::{file_utils::filter_by_extension, package::Package};

pub fn process_packages(
    packages: HashMap<String, Vec<PathBuf>>,
    dep_name: &str,
) -> Result<Vec<Vec<String>>, String> {
    let mut found = Vec::new();

    for (_, value) in packages {
        if let Some(toml) = filter_by_extension(&value, "toml") {
            // Parse .toml
            let toml_file = fs::read_to_string(&toml).map_err(|e| e.to_string())?;
            let parsed = Package::parse_toml(
                &toml_file,
                dep_name,
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
                let parsed =
                    Package::parse_lock(&lock_file, dep_name, lock.to_str().unwrap(), package_name);
                found.extend(parsed);
            }
        }
    }
    found.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(found
        .into_iter()
        .map(|package| {
            let mut res = Vec::new();
            res.push(package.name);
            res.push(package.dep_version);
            res.push(package.path);
            res
        })
        .collect())
}