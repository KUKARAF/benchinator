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

        // Get test command from config
        let test_command = self.config["docker"]["test_command"]
            .as_array()
            .ok_or_else(|| "Docker test_command not found in config".to_string())?;
        
        let test_args: Vec<&str> = test_command
            .iter()
            .map(|v| v.as_str().unwrap_or_default())
            .collect();

        // Run test with the image
        let mut docker_args = vec!["run", "--rm", image];
        docker_args.extend(test_args.iter());

        let run_output = Command::new("docker")
            .args(&docker_args)
            .output()
            .map_err(|e| format!("Failed to run docker container: {}", e))?;

        if !run_output.status.success() {
            return Err(String::from_utf8_lossy(&run_output.stderr).into_owned());
        }

        println!("Docker test completed successfully");
        Ok(())
    }
}

