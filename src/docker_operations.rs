use std::process::Command;
use std::fs;
use toml::Value;

pub struct DockerOperations {
    config: Value,
}

impl DockerOperations {
    pub fn new() -> Result<Self, String> {
        let config_str = fs::read_to_string("config.toml")
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Value = toml::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        Ok(DockerOperations { config })
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing docker operation...");
        
        // Get image from config
        let image = self.config["docker"]["image"]
            .as_str()
            .ok_or_else(|| "Docker image not found in config".to_string())?;

        // Pull the image
        let pull_output = Command::new("docker")
            .args(&["pull", image])
            .output()
            .map_err(|e| format!("Failed to pull docker image: {}", e))?;

        if !pull_output.status.success() {
            return Err(String::from_utf8_lossy(&pull_output.stderr).into_owned());
        }

        // Run a simple test with the image
        let run_output = Command::new("docker")
            .args(&["run", "--rm", image, "python", "--version"])
            .output()
            .map_err(|e| format!("Failed to run docker container: {}", e))?;

        if !run_output.status.success() {
            return Err(String::from_utf8_lossy(&run_output.stderr).into_owned());
        }

        println!("Docker test completed successfully");
        Ok(())
    }
}

