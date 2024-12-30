use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;
use toml::{Table, Value};

use crate::package::Row;

pub struct PackageFiles {
    pub ctoml: Option<PathBuf>,
    pub clock: Option<PathBuf>,
}

impl Default for PackageFiles {
    fn default() -> Self {
        Self {
            ctoml: Default::default(),
            clock: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed { version: String },
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    // #[serde(flatten)]
    // pub dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
pub struct LockFile {
    pub package: Vec<Package>,
}

pub fn process_packages(
    packages: HashMap<String, PackageFiles>,
    dep_name: &str,
) -> Result<Vec<Vec<String>>, String> {
    let mut found = Vec::new();

    for (_, value) in packages {
        if let Some(ref toml) = value.ctoml {
            // Parse .toml
            let toml_file = fs::read_to_string(toml).map_err(|e| e.to_string())?;
            let parsed = parse_toml(
                &toml_file,
                dep_name,
                toml.to_str().expect("Path must be a file"),
            );

            let package_name;
            if let Some(parsed) = parsed {
                package_name = parsed.name.clone();
                found.push(parsed);
            } else {
                package_name = parse_name(&toml_file);
            }

            // parse .lock
            if let Some(ref lock) = value.clock {
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
            let res = vec![package.name, package.dep_version, package.path];
            res
        })
        .collect())
}

fn parse_dependency(dep: &Value, package_name: &str, path: &str) -> Row {
    let version = dep.get("version").unwrap_or(dep);

    Row {
        name: package_name.to_string(),
        dep_version: version
            .as_str()
            .unwrap_or("Dependency is not a String")
            .to_string(),
        path: path.to_owned(),
    }
}

fn parse_name(contents: &str) -> String {
    let value = contents.parse::<Table>();
    if let Ok(table) = value {
        table
            .get("package")
            .and_then(|pack| pack.get("name"))
            .and_then(Value::as_str)
            .unwrap_or("Package name not found")
            .to_string()
    } else {
        "Package name not found".to_string()
    }
}

fn parse_toml(contents: &str, dependency_name: &str, path: &str) -> Option<Row> {
    let value = contents.parse::<Table>();

    if let Ok(table) = value {
        let package_name = table
            .get("package")
            .and_then(|pack| pack.get("name"))
            .and_then(Value::as_str)
            .unwrap_or("package name not found");

        for key in ["dependencies", "dep-dependencies"] {
            if let Some(deps) = table.get(key) {
                if let Some(dep) = deps.get(dependency_name) {
                    return Some(parse_dependency(dep, package_name, path));
                }
            }
        }
    } else {
        println!("Unparseable file: {:?}", path);
    }
    None
}

fn parse_lock2(
    contents: &str,
    dependency_name: &str,
    path: &str,
    package_name: String,
) -> Vec<Row> {
    let mut res = Vec::new();
    let parsed: Result<LockFile, _> = toml::from_str(contents);
    if let Ok(lock_file) = parsed {
        for package in lock_file.package {
            if package.name == dependency_name {
                res.push(Row {
                    name: package_name.clone(),
                    dep_version: package.version,
                    path: path.to_owned(),
                });
            }
        }
    }
    res
}

fn parse_lock(contents: &str, dependency_name: &str, path: &str, package_name: String) -> Vec<Row> {
    let mut res = Vec::new();
    if let Ok(table) = contents.parse::<Table>() {
        if let Some(Value::Array(packages)) = table.get("package") {
            for package in packages {
                if let Some(Value::String(name)) = package.get("name") {
                    if name == dependency_name {
                        let version = package
                            .get("version")
                            .and_then(Value::as_str)
                            .unwrap_or("Version not found")
                            .to_owned();
                        res.push(Row {
                            name: package_name.clone(),
                            dep_version: version,
                            path: path.to_owned(),
                        });
                    }
                }
            }
        }
    } else {
        println!("Unparseable file: {:?}", path);
    }
    res
}

#[cfg(test)]
mod test {
    use super::LockFile;

    #[test]
    fn test_deserialize() {
        let toml_str = r#"
        [[package]]
        name = "anstyle"
        version = "1.0.10"
        source = "registry+https://github.com/rust-lang/crates.io-index"
        checksum = "55cc3b69f167a1ef2e161439aa98aed94e6028e5f9a59be9a6ffb47aef1651f9"

        [[package]]
        name = "anstyle-parse"
        version = "0.2.6"
        source = "registry+https://github.com/rust-lang/crates.io-index"
        checksum = "3b2d16507662817a6a20a9ea92df6652ee4f94f914589377d69f3b21bc5798a9"
        dependencies = [
         "utf8parse",
        ]

        [[package]]
        name = "anstyle-query"
        version = "1.1.2"
        source = "registry+https://github.com/rust-lang/crates.io-index"
        checksum = "79947af37f4177cfead1110013d678905c37501914fba0efea834c3fe9a8d60c"
        dependencies = [
         "windows-sys",
        ]
        "#;

        let toml: LockFile = toml::from_str(toml_str).unwrap();
        dbg!(toml);
    }
}
