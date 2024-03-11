use std::{fs, path::PathBuf};

use crate::models::types::{Config, Project};

pub fn add_project(
    path: &String,
    home_dir: &PathBuf,
    config: &mut Config,
    pj_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = PathBuf::from(path);
    if !dir_path.is_dir() {
        return Err(format!("{} is not a valid directory.", path).into());
    }
    let full_path = match fs::canonicalize(&dir_path) {
        Ok(p) => p,
        Err(err) => return Err(format!("Error obtaining full path: {}", err).into()),
    };
    let relative_path = full_path.strip_prefix(home_dir)?;
    let new_project = Project {
        name: full_path
            .file_name()
            .expect("Cannot get directory name")
            .to_str()
            .expect("")
            .to_string(),
        path_from_home: relative_path.to_str().expect("").to_string(),
    };
    config.projects.push(new_project);
    let serialized_config = serde_json::to_string_pretty(&config)?;
    fs::write(pj_file, serialized_config)?;

    println!("Project added successfully.");
    Ok(())
}
