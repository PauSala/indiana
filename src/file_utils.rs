use std::{fs, path::PathBuf};

pub fn collect_files(
    path: &PathBuf,
    files: &mut Vec<PathBuf>,
    target_files: &[&str],
) -> Result<(), String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files, target_files)?;
        } else if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if target_files.contains(&file_name) {
                files.push(path);
            }
        }
    }

    Ok(())
}

pub fn filter_by_extension<'a>(files: &'a Vec<PathBuf>, target: &'a str) -> Option<&'a PathBuf> {
    files.iter().find(|path| {
        if let Some(ext) = path.extension() {
            ext == target
        } else {
            false
        }
    })
}
