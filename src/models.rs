use crate::dto::GithubEventTypes;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub repo: String,
    pub command: Option<String>,
    #[serde(default = "Vec::default")]
    pub args: Vec<String>,
    pub secret: Option<String>,
    #[serde(default = "Vec::default")]
    pub events: Vec<GithubEventTypes>,
    pub repo_directory: String,
    #[serde(default = "default_working_directory")]
    pub working_directory: String,
    #[serde(default = "default_branch")]
    pub branch: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub repos: Vec<Repo>,
    pub host: String,
    #[serde(default = "default_git_directory")]
    pub git: String,
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_git_directory() -> String {
    if cfg!(target_os = "windows") {
        "C:\\Program Files\\Git\\cmd\\git.exe".to_string()
    } else {
        "/usr/bin/git".to_string()
    }
}

fn default_working_directory() -> String {
    ".".to_string()
}

