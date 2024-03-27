use std::{fs, io, path::PathBuf};

use crate::models::types::Config;

pub fn go_to_project(
    config: &mut Config,
    home_dir: &PathBuf,
    pj_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Available projects:");
    for (index, project) in config.projects.iter().enumerate() {
        println!("{}. {}", index, project.name);
    }
    println!("Please pick a project (index):");

    // Create a mutable string to store the user's input
    let mut selected_project = String::new();

    // Read the user's input from stdin
    io::stdin()
        .read_line(&mut selected_project)
        .expect("Failed to read line");

    // Parse the input as usize
    let selected_index: usize = match selected_project.trim().parse() {
        Ok(index) => index,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return Ok(());
        }
    };

    // Check if the index is valid
    if selected_index >= config.projects.len() {
        println!("Invalid index. Please enter a valid index.");
        return Ok(());
    }

    // Get the project path by index
    let selected_project = &config.projects[selected_index];
    let project_path = home_dir.join(&selected_project.path_from_home);

    // Update last_opened in config
    config.last_opened = Some(project_path.display().to_string());

    // Serialize config to JSON
    let json_config = serde_json::to_string_pretty(config)?;

    // Write JSON to file
    fs::write(&pj_file, json_config)?;
    Ok(())
}
