use std::process::Command;

pub struct GitOperations;

impl GitOperations {
    pub fn new() -> Self {
        GitOperations
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing git operation...");
        
        // Check git status
        let output = Command::new("git")
            .arg("status")
            .output()
            .map_err(|e| format!("Failed to execute git status: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }

        Ok(())
    }
}

