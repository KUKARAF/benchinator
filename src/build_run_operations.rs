use std::process::Command;

pub struct BuildRunOperations;

impl BuildRunOperations {
    pub fn new() -> Self {
        BuildRunOperations
    }

    pub fn perform_operation(&self) -> Result<(), String> {
        println!("Performing build and run operations...");
        
        // Build and run a C project
        self.build_and_run_c_project()?;

        // Start a Python Django application
        self.start_django_application()?;

        Ok(())
    }

    fn build_and_run_c_project(&self) -> Result<(), String> {
        // Assuming a simple C project in the current directory
        Command::new("gcc")
            .args(&["main.c", "-o", "program"])
            .output()
            .map_err(|e| format!("Failed to build C project: {}", e))?;

        Command::new("./program")
            .output()
            .map_err(|e| format!("Failed to run C program: {}", e))?;

        println!("C project built and run successfully.");
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
