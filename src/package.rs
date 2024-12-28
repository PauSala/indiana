use toml::{Table, Value};

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub dep_version: String,
    pub path: String,
}

impl Package {
    pub fn parse_name(contents: &str) -> String {
        let value = contents.parse::<Table>();
        if let Ok(table) = value {
            let package = &table["package"]["name"].as_str();
            return package.unwrap().to_string();
        }
        return "".to_string();
    }

    pub fn parse_toml(contents: &str, dependency_name: &str, path: &str) -> Option<Self> {
        let value = contents.parse::<Table>();

        if let Ok(table) = value {
            let package = &table["package"]["name"].as_str();

            let parse_dependency = |dep: &Value| {
                let version = dep.get("version").unwrap_or(dep);

                return Some(Package {
                    name: package.unwrap_or("Package name not found").to_string(),
                    dep_version: version.as_str().unwrap().to_string(),
                    path: path.to_owned(),
                });
            };

            let dev_deps = &table.get("dependencies");
            if let Some(deps) = dev_deps {
                if let Some(dep) = deps.get(dependency_name) {
                    return parse_dependency(dep);
                }
            }

            let dev_deps = &table.get("dep-dependencies");
            if let Some(deps) = dev_deps {
                if let Some(dep) = deps.get(dependency_name) {
                    return parse_dependency(dep);
                }
            }
        } else {
            println!("{:?}", value);
        }
        None
    }

    pub fn parse_lock(
        contents: &str,
        dependency_name: &str,
        path: &str,
        package_name: String,
    ) -> Vec<Package> {
        let value = contents.parse::<Table>();
        let mut res = Vec::new();

        if let Ok(table) = value {
            match &table["package"] {
                Value::Array(vec) => {
                    for package in vec {
                        let version = package.get("version");
                        let name = package.get("name");
                        if let Some(Value::String(name)) = name {
                            if name == dependency_name {
                                res.push(Package {
                                    name: package_name.clone(),
                                    dep_version: version
                                        .expect("pakcage should have a version")
                                        .as_str()
                                        .unwrap()
                                        .to_owned(),
                                    path: path.to_owned(),
                                });
                            }
                        }
                    }
                }
                _ => return res,
            }
        } else {
            println!("{:?}", value);
        }
        res
    }
}
