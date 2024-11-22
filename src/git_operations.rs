use std::process::Command;
use std::fs::{self, File};
use std::io::Write;
use rand::Rng;

pub struct GitOperations;

impl GitOperations {
    pub fn new() -> Self {
        GitOperations
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing git operations...");
        
        // Initialize a new repository
        self.init_repo()?;

        // Create and commit 50 random text files
        self.create_and_commit_files(50)?;

        Ok(())
    }

    fn init_repo(&self) -> Result<(), String> {
        Command::new("git")
            .args(&["init"])
            .output()
            .map_err(|e| format!("Failed to initialize git repository: {}", e))?;

        println!("Git repository initialized.");
        Ok(())
    }

    fn create_and_commit_files(&self, count: usize) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        for i in 0..count {
            let filename = format!("random_file_{}.txt", i);
            let content: String = (0..100).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

            let mut file = File::create(&filename)
                .map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;
            file.write_all(content.as_bytes())
                .map_err(|e| format!("Failed to write to file '{}': {}", filename, e))?;

            Command::new("git")
                .args(&["add", &filename])
                .output()
                .map_err(|e| format!("Failed to add file '{}' to git: {}", filename, e))?;

            Command::new("git")
                .args(&["commit", "-m", &format!("Add {}", filename)])
                .output()
                .map_err(|e| format!("Failed to commit file '{}': {}", filename, e))?;
        }

        println!("{} files created and committed.", count);
        Ok(())
    }
}

