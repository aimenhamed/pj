use clap::{arg, Command};
use dirs;
use std::fs;
mod commands;
use commands::{add::add_project, go::go_to_project, remove::remove_project};
mod models;
use models::types::Config;

fn cli() -> Command {
    Command::new("pj")
        .version("1.0")
        .author("pj")
        .about("Project manager")
        .subcommand(
            Command::new("add")
                .about("Add a project")
                .arg(arg!(<PATH> "Path of project")),
        )
        .subcommand(
            Command::new("remove")
                .alias("rm")
                .about("Remove a project")
                .arg(arg!(<NAME> "Project name")),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Unable to determine the home directory.")?;
    let pj_file = home_dir.join(".pj.json");
    if !pj_file.exists() {
        fs::write(&pj_file, "{}")?;
    }
    let pj_contents = fs::read_to_string(&pj_file)?;
    let mut config: Config = serde_json::from_str(&pj_contents)?;

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("required");
            add_project(path, &home_dir, &mut config, &pj_file).expect("Failed to add project");
        }
        Some(("remove", sub_matches)) => {
            let project_name = sub_matches.get_one::<String>("NAME").expect("required");
            remove_project(project_name.to_owned(), &mut config, &pj_file)
                .expect("Failed to remove project");
        }
        _ => go_to_project(&mut config, &home_dir, &pj_file).expect("Failed to go to project"),
    }
    Ok(())
}
