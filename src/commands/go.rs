use std::{fs, io, path::PathBuf};

use crate::models::types::Config;

pub fn go_to_project(
    config: &mut Config,
    home_dir: &PathBuf,
    pj_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Available projects:");
    for project in &config.projects {
        println!("{}", project.name);
    }
    println!("Please pick a project:");

    // Create a mutable string to store the user's input
    let mut project_name_to_find = String::new();

    // Read the user's input from stdin
    io::stdin()
        .read_line(&mut project_name_to_find)
        .expect("Failed to read line");

    // Trim whitespace and newline characters from the input
    let project_name_to_find = project_name_to_find.trim();

    // Find the selected project in the configuration
    let found_project_path = config
        .projects
        .iter()
        .find(|project| project.name == project_name_to_find)
        .map(|project| home_dir.join(&project.path_from_home));

    // If the selected project was found, return its path
    if let Some(project_path) = found_project_path {
        config.last_opened = Some(project_path.display().to_string());
        let json_config = serde_json::to_string_pretty(&config)?;
        fs::write(&pj_file, json_config)?;
    } else {
        Err(format!(
            "Project '{}' not found in configuration.",
            project_name_to_find
        ))?
    }
    Ok(())
}
