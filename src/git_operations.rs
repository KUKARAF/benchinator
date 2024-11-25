use std::process::Command;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    git: GitConfig,
}

impl GitOperations {
    fn remove_random_files(&self, count: usize) -> Result<(), String> {
        let files = fs::read_dir("artifacts")
            .map_err(|e| format!("Failed to read artifacts directory: {}", e))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "txt"))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();
        let files_to_remove = files.choose_multiple(&mut rng, count).collect::<Vec<_>>();

        for file in files_to_remove {
            let filename = file.file_name();
            let filename_str = filename.to_string_lossy();
            
            fs::remove_file(file.path())
                .map_err(|e| format!("Failed to remove file '{}': {}", filename_str, e))?;

            Command::new("git")
                .current_dir("artifacts")
                .args(&["rm", &filename_str])
                .output()
                .map_err(|e| format!("Failed to git rm file '{}': {}", filename_str, e))?;

            Command::new("git")
                .current_dir("artifacts")
                .args(&["commit", "-m", &format!("Remove {}", filename_str)])
                .output()
                .map_err(|e| format!("Failed to commit removal of '{}': {}", filename_str, e))?;
        }

        println!("{} files randomly removed from the repository.", count);
        Ok(())
    }
}

#[derive(Deserialize)]
struct GitConfig {
    files_count: usize,
    files_to_remove: usize,
    files_to_add: usize,
    branches: Vec<String>,
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

        // Create and switch to feature branch
        self.create_test_branch()?;

        // Remove random files
        self.remove_random_files(self.config.git.files_to_remove)?;

        // Add new files
        self.create_and_commit_files(self.config.git.files_to_add)?;

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

    fn create_test_branch(&self) -> Result<(), String> {
        // Use the first feature branch from the config
        let branch_name = self.config.git.branches.iter()
            .find(|b| b.starts_with("feature/"))
            .ok_or_else(|| "No feature branch found in config".to_string())?;

        Command::new("git")
            .current_dir("artifacts")
            .args(&["checkout", "-b", branch_name])
            .output()
            .map_err(|e| format!("Failed to create test branch: {}", e))?;

        println!("Created and switched to feature branch '{}'", branch_name);
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

