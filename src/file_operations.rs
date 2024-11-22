use std::fs;
use std::io;

pub struct FileOperations;

impl FileOperations {
    pub fn new() -> Self {
        FileOperations
    }

    pub fn perform_operation(&self) -> io::Result<()> {
        println!("Performing file operation...");
        
        // Create a temporary file
        let temp_file = "temp_benchmark_file.txt";
        fs::write(temp_file, "Benchmark test content")?;

        // Read the file
        let _content = fs::read_to_string(temp_file)?;

        // Delete the file
        fs::remove_file(temp_file)?;

        Ok(())
    }
}

