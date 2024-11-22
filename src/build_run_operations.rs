use std::process::Command;

pub struct BuildRunOperations;

impl BuildRunOperations {
    pub fn new() -> Self {
        BuildRunOperations
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing build and run operations...");
        
        // Start a Python Django application
        self.start_django_application()?;

        Ok(())
    }

    fn start_django_application(&self) -> Result<(), String> {
        // Assuming a Django project in the current directory
        Command::new("python")
            .args(&["manage.py", "runserver"])
            .output()
            .map_err(|e| format!("Failed to start Django application: {}", e))?;

        println!("Django application started successfully.");
        Ok(())
    }
}
