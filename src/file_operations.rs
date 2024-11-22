use std::io::{self, Read, Write};
use std::fs::{self, File};
use rand::Rng;
use sha2::{Sha256, Digest};

pub struct FileOperations;

impl FileOperations {
    pub fn new() -> Self {
        FileOperations
    }

    pub fn perform_operation(&self) -> io::Result<()> {
        println!("Performing file operations...");
        
        // Write 500MB random file
        let file_size = 500 * 1024 * 1024; // 500MB
        let temp_file = "temp_benchmark_file.bin";
        self.write_random_file(temp_file, file_size)?;

        // Read 500MB file
        self.read_file(temp_file)?;

        // Load 500MB to RAM
        let data = self.load_to_ram(temp_file)?;

        // Calculate file hash from disk
        let disk_hash = self.calculate_file_hash(temp_file)?;
        println!("File hash from disk: {}", disk_hash);

        // Calculate file hash from RAM
        let ram_hash = self.calculate_ram_hash(&data);
        println!("File hash from RAM: {}", ram_hash);

        // Clean up
        fs::remove_file(temp_file)?;

        Ok(())
    }

    fn write_random_file(&self, filename: &str, size: usize) -> io::Result<()> {
        let mut file = File::create(filename)?;
        let mut rng = rand::thread_rng();
        let mut buffer = [0u8; 8192];

        for _ in 0..(size / buffer.len()) {
            rng.fill(&mut buffer[..]);
            file.write_all(&buffer)?;
        }

        let remainder = size % buffer.len();
        if remainder > 0 {
            rng.fill(&mut buffer[..remainder]);
            file.write_all(&buffer[..remainder])?;
        }

        Ok(())
    }

    fn read_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = [0u8; 8192];
        
        while file.read(&mut buffer)? != 0 {}
        
        Ok(())
    }

    fn load_to_ram(&self, filename: &str) -> io::Result<Vec<u8>> {
        fs::read(filename)
    }

    fn calculate_file_hash(&self, filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    fn calculate_ram_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

