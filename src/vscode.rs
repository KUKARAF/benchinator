use std::fs;
use std::process::Command;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    git: GitConfig,
}

#[derive(Deserialize)]
struct GitConfig {
    branches: Vec<String>,
}

pub struct VsCodeOperations;

impl VsCodeOperations {
    pub fn new() -> Self {
        VsCodeOperations
    }

    pub fn open_branches(&self) -> Result<(), String> {
        // Read config file
        let config_str = fs::read_to_string("config.toml")
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Config = toml::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        // First open VS Code in artifacts directory
        Command::new("code")
            .arg("artifacts")
            .output()
            .map_err(|e| format!("Failed to open VS Code: {}", e))?;

        // Change to artifacts directory
        if !Path::new("artifacts").exists() {
            return Err("Artifacts directory does not exist".to_string());
        }

        // For each branch in config
        for branch in &config.git.branches {
            // Checkout branch
            Command::new("git")
                .current_dir("artifacts")
                .args(&["checkout", branch])
                .output()
                .map_err(|e| format!("Failed to checkout branch {}: {}", branch, e))?;

            // Get list of .txt files
            let files = fs::read_dir("artifacts")
                .map_err(|e| format!("Failed to read artifacts directory: {}", e))?
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "txt"))
                .take(3) // Only take first 3 files
                .collect::<Vec<_>>();

            // Open first 3 files in VS Code
            for file in files {
                Command::new("code")
                    .current_dir("artifacts")
                    .arg(file.path())
                    .output()
                    .map_err(|e| format!("Failed to open file in VS Code: {}", e))?;
            }
        }

        println!("Opened VS Code with files from all branches");
        Ok(())
    }
}
