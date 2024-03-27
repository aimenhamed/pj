use crate::models::types::Config;

pub fn list_projects(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Available projects:");
    for (index, project) in config.projects.iter().enumerate() {
        println!("{}. {}", index, project.name);
    }
    Ok(())
}
