use toml::{Table, Value};

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub dep_version: String,
    pub path: String,
}

impl Package {
    fn parse_dependency(dep: &Value, package_name: &str, path: &str) -> Self {
        let version = dep.get("version").unwrap_or(dep);

        Package {
            name: package_name.to_string(),
            dep_version: version
                .as_str()
                .unwrap_or("Dependency is not a String")
                .to_string(),
            path: path.to_owned(),
        }
    }

    pub fn parse_name(contents: &str) -> String {
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

    pub fn parse_toml(contents: &str, dependency_name: &str, path: &str) -> Option<Self> {
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
                        return Some(Package::parse_dependency(dep, package_name, path));
                    }
                }
            }
        } else {
            println!("Unparseable file: {:?}", value);
        }
        None
    }

    pub fn parse_lock(
        contents: &str,
        dependency_name: &str,
        path: &str,
        package_name: String,
    ) -> Vec<Package> {
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
                            res.push(Package {
                                name: package_name.clone(),
                                dep_version: version,
                                path: path.to_owned(),
                            });
                        }
                    }
                }
            }
        } else {
            println!("Unparseable file: {:?}", contents);
        }
        res
    }
}
