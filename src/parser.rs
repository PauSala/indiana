pub mod data;

use data::{CLockFile, CTomlFile, Dependency, MatchInfo, PackageFiles, Row};
use hashbrown::HashMap;
use std::fs;

use crate::file_utils::{LOCK, TOML};

#[derive(Default)]
pub struct FileParser;

impl FileParser {
    pub fn new() -> Self {
        FileParser
    }

    pub fn parse(
        &self,
        files: HashMap<String, PackageFiles>,
        target_dep: &str,
    ) -> Result<Vec<Row>, String> {
        let mut found = Vec::new();

        for (_, package) in files {
            if let Some(ref toml) = package.ctoml {
                // Parse .toml
                let toml_file = fs::read_to_string(toml).map_err(|e| e.to_string())?;
                let parsed = self.parse_toml(
                    &toml_file,
                    target_dep,
                    toml.to_str().expect("Path must be a file"),
                );

                let package_name;
                if let Some(parsed) = &parsed.iter().find(|e| e.dep_version != "-") {
                    package_name = parsed.package_name.clone();
                } else {
                    package_name = self.parse_name(&toml_file);
                }

                found.extend(parsed);

                // parse .lock
                if let Some(ref lock) = package.clock {
                    let lock_file = fs::read_to_string(lock).map_err(|e| e.to_string())?;
                    let parsed = self.parse_lock(
                        &lock_file,
                        target_dep,
                        lock.to_str().unwrap(),
                        package_name,
                    );
                    found.extend(parsed);
                }
            }
        }
        found.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(found
            .into_iter()
            .map(|package| {
                let res: [String; 3] = [package.package_name, package.dep_version, package.path];
                res
            })
            .collect())
    }

    fn parse_toml(&self, contents: &str, target_dep: &str, path: &str) -> Vec<MatchInfo> {
        let mut res = Vec::new();
        let parsed: Result<CTomlFile, _> = toml::from_str(contents);
        if let Ok(toml) = parsed {
            let package_name = toml
                .package
                .map(|package| package.name)
                .unwrap_or_else(|| "-".to_string());

            for deps in [toml.dependencies, toml.dev_dependencies] {
                if let Some(found) = self.parse_dependencies(deps, target_dep, path, &package_name)
                {
                    res.push(found);
                }
            }
            if let Some(data) = toml.target.and_then(|target| target.targets) {
                for (_, target) in data {
                    for deps in [target.dependencies, target.dev_dependencies] {
                        if let Some(found) =
                            self.parse_dependencies(deps, target_dep, path, &package_name)
                        {
                            res.push(found);
                        }
                    }
                }
            }
            return res;
        } else {
            match parsed {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Unparseable file: {:?} {e}", path);
                }
            };
        }
        res
    }

    fn parse_lock(
        &self,
        contents: &str,
        dependency_name: &str,
        path: &str,
        package_name: String,
    ) -> Vec<MatchInfo> {
        let mut res = Vec::new();
        let parsed: Result<CLockFile, _> = toml::from_str(contents);
        if let Ok(lock_file) = parsed {
            for package in lock_file.package {
                if package.name == dependency_name {
                    res.push(MatchInfo {
                        package_name: package_name.clone(),
                        dep_version: package.version,
                        path: path.to_owned(),
                        extension: LOCK.to_string(),
                    });
                }
            }
        }
        res
    }

    fn parse_name(&self, contents: &str) -> String {
        toml::from_str::<CTomlFile>(contents)
            .ok()
            .and_then(|toml| toml.package.map(|package| package.name))
            .unwrap_or_else(|| "-".to_string())
    }

    fn parse_dependencies(
        &self,
        dependencies: Option<HashMap<String, Dependency>>,
        target_dep: &str,
        path: &str,
        package_name: &str,
    ) -> Option<MatchInfo> {
        if let Some(dependencies) = dependencies {
            for (dep_name, dep) in dependencies {
                if dep_name == target_dep {
                    match dep {
                        data::Dependency::Simple(version) => {
                            return Some(MatchInfo {
                                package_name: package_name.to_owned(),
                                dep_version: version,
                                path: path.to_string(),
                                extension: TOML.to_string(),
                            })
                        }
                        data::Dependency::Detailed(dependency_details) => {
                            return Some(MatchInfo {
                                package_name: package_name.to_owned(),
                                dep_version: dependency_details.version.unwrap_or("-".to_string()),
                                path: path.to_string(),
                                extension: TOML.to_string(),
                            })
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use toml::Table;

    use crate::parser::data::CLockFile;

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

        let toml: CLockFile = toml::from_str(toml_str).unwrap();
        dbg!(toml);
    }

    #[test]
    fn test_deserialize2() {
        let toml_str = r#"
            [package]
            name = "core_simd"
            version = "0.1.0"
            edition = "2021"
            homepage = "https://github.com/rust-lang/portable-simd"
            repository = "https://github.com/rust-lang/portable-simd"
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

        let toml: Table = toml::from_str(toml_str).unwrap();
        dbg!(toml);
    }
}
