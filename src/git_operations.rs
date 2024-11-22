use std::process::Command;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use rand::Rng;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    git: GitConfig,
}

#[derive(Deserialize)]
struct GitConfig {
    files_count: usize,
}

pub struct GitOperations {
    config: Config,
}

impl GitOperations {
    pub fn new() -> Result<Self, String> {
        let config_str = fs::read_to_string("config.toml")
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Config = toml::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        Ok(GitOperations { config })
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing git operations...");
        
        // Create artifacts directory
        self.create_artifacts_dir()?;

        // Initialize a new repository in artifacts directory
        self.init_repo()?;

        // Create and commit files based on config
        self.create_and_commit_files(self.config.git.files_count)?;

        Ok(())
    }

    fn create_artifacts_dir(&self) -> Result<(), String> {
        fs::create_dir_all("artifacts")
            .map_err(|e| format!("Failed to create artifacts directory: {}", e))?;
        println!("Artifacts directory created.");
        Ok(())
    }

    fn init_repo(&self) -> Result<(), String> {
        Command::new("git")
            .current_dir("artifacts")
            .args(&["init"])
            .output()
            .map_err(|e| format!("Failed to initialize git repository: {}", e))?;

        println!("Git repository initialized in artifacts directory.");
        Ok(())
    }

    fn create_and_commit_files(&self, count: usize) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        for i in 0..count {
            let filename = format!("random_file_{}.txt", i);
            let filepath = Path::new("artifacts").join(&filename);
            let content: String = (0..100).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

            let mut file = File::create(&filepath)
                .map_err(|e| format!("Failed to create file '{}': {}", filepath.display(), e))?;
            file.write_all(content.as_bytes())
                .map_err(|e| format!("Failed to write to file '{}': {}", filepath.display(), e))?;

            Command::new("git")
                .current_dir("artifacts")
                .args(&["add", &filename])
                .output()
                .map_err(|e| format!("Failed to add file '{}' to git: {}", filename, e))?;

            Command::new("git")
                .current_dir("artifacts")
                .args(&["commit", "-m", &format!("Add {}", filename)])
                .output()
                .map_err(|e| format!("Failed to commit file '{}': {}", filename, e))?;
        }

        println!("{} files created and committed in artifacts directory.", count);
        Ok(())
    }
}

