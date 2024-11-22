use std::process::Command;

pub struct DockerOperations;

impl DockerOperations {
    pub fn new() -> Self {
        DockerOperations
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing docker operation...");
        
        // Check docker version
        let output = Command::new("docker")
            .arg("version")
            .output()
            .map_err(|e| format!("Failed to execute docker version: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }

        Ok(())
    }
}

