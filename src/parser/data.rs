use hashbrown::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct OutputRow {
    pub package_name: String,
    pub dep_version: String,
    pub path: String,
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
    use crate::parser::data::{CLockFile, CTomlFile};

    #[test]
    fn deserialize_simple() {
        let toml_content = r#"
        [package]
        name = "test"
        version = "0.1.0"
        edition = "2021"
            
        [dependencies]
        serde = "1.0"

        [dev-dependencies]
        io = "1.0"
        "#;

        let parsed: CTomlFile = toml::from_str(toml_content).unwrap();
        assert!(parsed.dependencies.is_some());
        assert!(parsed.dev_dependencies.is_some());
    }

    #[test]
    fn deserialize_nested() {
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
        assert!(parsed.dependencies.is_some());
        assert!(parsed.dev_dependencies.is_some());
        assert!(parsed.dependencies.unwrap().get("toml").is_some());
    }

    #[test]
    fn test_deserialize_targets() {
        let toml_str = r#"
            [package]
            name = "core_simd"
            version = "0.1.0"
            edition = "2021"
            homepage = ""
            repository = ""
            keywords = ["core", "simd", "intrinsics"]
            categories = ["hardware-support", "no-std"]
            license = "MIT OR Apache-2.0"
                
            [features]
            default = ["as_crate"]
            as_crate = []
            std = []
            all_lane_counts = []
                
            [target.'cfg(target_arch = "wasm32")'.dev-dependencies]
            wasm-bindgen = "0.2"
            wasm-bindgen-test = "0.3"
                
            [dev-dependencies.proptest]
            version = "0.10"
            default-features = false
            features = ["alloc"]
                
            [dev-dependencies.test_helpers]
            path = "../test_helpers"
                
            [dev-dependencies]
            std_float = { path = "../std_float/", features = ["as_crate"] }
        "#;

        let toml: CTomlFile = toml::from_str(toml_str).unwrap();
        assert!(toml.dev_dependencies.is_some());
        assert!(toml.target.is_some());
        assert!(toml
            .target
            .unwrap()
            .targets
            .is_some_and(|t| t.get("cfg(target_arch = \"wasm32\")").is_some()));
    }

    #[test]
    fn deserialize_lock() {
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

        let lock: CLockFile = toml::from_str(toml_str).unwrap();
        assert!(lock.package.len() == 3);
    }
}
