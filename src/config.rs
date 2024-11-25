use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub file_size_mb: usize,
    pub num_git_files: usize,
    pub c_project_url: String,
    pub django_project_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            file_size_mb: 500,
            num_git_files: 50,
            c_project_url: "https://github.com/example/c-project.git".to_string(),
            django_project_url: "https://github.com/example/django-project.git".to_string(),
        }
    }
}

pub fn load_or_create_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.toml");

    if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    } else {
        let config = Config::default();
        let toml_string = toml::to_string(&config)?;
        fs::write(config_path, toml_string)?;
        Ok(config)
    }
}

