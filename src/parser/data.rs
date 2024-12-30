use hashbrown::HashMap;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FoundDependency {
    pub package_name: String,
    pub dep_version: String,
    pub extension: String,
    pub path: String,
}

pub type Row = [String; 3];

#[derive(Default)]
pub struct PackageFiles {
    pub ctoml: Option<PathBuf>,
    pub clock: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DependencyDetails),
}

#[derive(Debug, Deserialize)]
pub struct DependencyDetails {
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    #[serde(rename = "dev-dependencies")]
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
pub struct Targets {
    #[serde(flatten)]
    pub targets: Option<HashMap<String, Target>>,
}

#[derive(Debug, Deserialize)]
pub struct CTomlFile {
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, Dependency>>,
    #[serde(rename = "dev-dependencies")]
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
    #[serde(rename = "target")]
    pub target: Option<Targets>,
}

#[derive(Debug, Deserialize)]
pub struct CLockFile {
    pub package: Vec<Package>,
}

#[cfg(test)]
mod test {
    use crate::parser::data::CTomlFile;

    #[test]
    fn serde() {
        let toml_content = r#"
        [package]
        name = "test"
        version = "0.1.0"
        edition = "2021"
            
        [dependencies]
        serde = "1.0"

        [dev-dependencies]
        io = "1.0"
    
        [dependencies.toml]
        features = ["derive"]
        version = "1.0.136"
        "#;

        let parsed: CTomlFile = toml::from_str(toml_content).unwrap();
        println!("{:#?}", parsed);
    }
}
