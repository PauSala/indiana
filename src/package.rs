use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub package_name: String,
    pub dep_version: String,
    pub path: String,
}

impl DependencyInfo {
    pub fn from_dependency(value: &Dependency, package_name: String, path: String) -> Self {
        match value {
            Dependency::Simple(dep) => DependencyInfo {
                package_name,
                dep_version: dep.to_string(),
                path,
            },
            Dependency::Detailed { version, .. } => DependencyInfo {
                package_name,
                dep_version: version.to_string(),
                path,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed {
        version: String,
        features: Vec<String>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Toml {
    pub package: Package,
    pub dependencies: Option<HashMap<String, Dependency>>,
    #[serde(rename = "dev-dependencies")]
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
pub struct Lock {
    pub package: Vec<Package>,
}

#[cfg(test)]
mod test {
    use crate::package::{Lock, Toml};

    #[test]
    fn test_deserialize_toml() {
        let toml: Toml = toml::from_str(
            r#"
        [package]
        name = "indiana"
        version = "0.1.0"
        edition = "2021"
        [dependencies]
        clap = { version = "4.5.23", features = ["derive"] }
        serde = { version = "1.0", features = ["derive"] }
        toml = "0.8.19"
        "#,
        )
        .unwrap();
        dbg!(toml);
    }

    #[test]
    fn test_deserialize_lock() {
        let toml: Lock = toml::from_str(
            r#"
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
        "#,
        )
        .unwrap();
        dbg!(toml);
    }
}
