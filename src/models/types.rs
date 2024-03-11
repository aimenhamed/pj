use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub path_from_home: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub last_opened: Option<String>,
    pub projects: Vec<Project>,
}
