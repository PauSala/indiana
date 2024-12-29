#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub dep_version: String,
    pub path: String,
}
