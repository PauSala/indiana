use crate::{file_utils::filter_by_extension, package::Package};
use std::{collections::HashMap, fs, path::PathBuf};
use toml::{Table, Value};


pub fn process_packages(
    packages: HashMap<String, Vec<PathBuf>>,
    dep_name: &str,
) -> Result<Vec<Vec<String>>, String> {
    let mut found = Vec::new();

    for (_, value) in packages {
        if let Some(toml) = filter_by_extension(&value, "toml") {
            // Parse .toml
            let toml_file = fs::read_to_string(toml).map_err(|e| e.to_string())?;
            let package = parse_toml(
                &toml_file,
                dep_name,
                toml.to_str().expect("Path must be a file"),
            );

            let package_name;
            if let Some(parsed) = package {
                package_name = parsed.package_name.clone();
                found.push(parsed);
            } else {
                package_name = parse_name(&toml_file);
            }

            // parse .lock
            if let Some(lock) = filter_by_extension(&value, "lock") {
                let lock_file = fs::read_to_string(lock).map_err(|e| e.to_string())?;
                let parsed = parse_lock(&lock_file, dep_name, lock.to_str().unwrap(), package_name);
                found.extend(parsed);
            }
        }
    }
    found.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(found
        .into_iter()
        .map(|package| {
            let res = vec![package.package_name, package.dep_version, package.path];
            res
        })
        .collect())
}

fn parse_name(contents: &str) -> String {
    if let Some(toml) = toml::from_str::<Toml>(contents).ok() {
        toml.package.name
    } else {
        "Package name not found".to_string()
    }
}

fn parse_toml(contents: &str, dependency_name: &str, path: &str) -> Option<DependencyInfo> {
    if let Some(toml) = toml::from_str::<Toml>(contents).ok() {
        let name = toml.package.name;
        if let Some(deps) = toml.dependencies {
            if let Some(dep) = deps.get(dependency_name) {
                return Some(DependencyInfo::from_dependency(dep, name, path.to_string()));
            }
        } else if let Some(deps) = toml.dev_dependencies {
            if let Some(dep) = deps.get(dependency_name) {
                return Some(DependencyInfo::from_dependency(dep, name, path.to_string()));
            }
        }
    }
    None
}

fn parse_lock(
    contents: &str,
    dependency_name: &str,
    path: &str,
    package_name: String,
) -> Vec<DependencyInfo> {
    let mut res = Vec::new();

    if let Some(lock) = toml::from_str::<Lock>(contents).ok() {
        for package in lock.package {
            if package.name == dependency_name {
                let version = package.version;
                res.push(DependencyInfo {
                    package_name: package_name.clone(),
                    dep_version: version,
                    path: path.to_owned(),
                });
            }
        }
    }
    res
}
