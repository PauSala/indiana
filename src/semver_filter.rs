use crate::parser::data::OutputRow;
use semver::{Version, VersionReq};

pub fn filter(filter: Option<VersionReq>, data: Vec<OutputRow>) -> Vec<OutputRow> {
    data.into_iter()
        .filter(|row| {
            if let Some(filter) = &filter {
                if let Ok(dep_version) = Version::parse(&row.dep_version) {
                    filter.matches(&dep_version)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect()
}
