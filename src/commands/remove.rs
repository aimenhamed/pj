use std::{fs, path::PathBuf};

use crate::models::types::Config;

pub fn remove_project(
    project_name: String,
    config: &mut Config,
    pj_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let pos = config.projects.iter().position(|x| x.name == project_name);

    if let None = pos {
        println!("No project with name found.");
        return Ok(());
    }

    if let Some(index) = pos {
        config.projects.remove(index);
    }

    let serialized_config = serde_json::to_string_pretty(&config)?;
    fs::write(pj_file, serialized_config)?;

    println!("Project removed successfully.");
    Ok(())
}
