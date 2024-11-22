use std::io::{self, Read, Write};
use std::fs::{self, File};
use rand::Rng;
use sha2::{Sha256, Digest};

pub struct FileOperationResults {
    pub write_time: u128,
    pub read_time: u128,
    pub ram_load_time: u128,
    pub disk_hash_time: u128,
    pub ram_hash_time: u128,
}

pub struct FileOperations;

impl FileOperations {
    pub fn new() -> Self {
        FileOperations
    }

    pub fn perform_operation(&self) -> io::Result<FileOperationResults> {
        println!("Performing file operations...");
        
        let file_size = 500 * 1024 * 1024; // 500MB
        let temp_file = "temp_benchmark_file.bin";
        
        // Measure write operation
        let write_start = std::time::Instant::now();
        self.write_random_file(temp_file, file_size)?;
        let write_time = write_start.elapsed().as_millis();
        println!("Write operation completed in {} ms", write_time);

        // Measure read operation
        let read_start = std::time::Instant::now();
        self.read_file(temp_file)?;
        let read_time = read_start.elapsed().as_millis();
        println!("Read operation completed in {} ms", read_time);

        // Measure RAM load operation
        let ram_load_start = std::time::Instant::now();
        let data = self.load_to_ram(temp_file)?;
        let ram_load_time = ram_load_start.elapsed().as_millis();
        println!("RAM load operation completed in {} ms", ram_load_time);

        // Measure disk hash calculation
        let disk_hash_start = std::time::Instant::now();
        let disk_hash = self.calculate_file_hash(temp_file)?;
        let disk_hash_time = disk_hash_start.elapsed().as_millis();
        println!("File hash from disk: {} (completed in {} ms)", disk_hash, disk_hash_time);

        // Measure RAM hash calculation
        let ram_hash_start = std::time::Instant::now();
        let ram_hash = self.calculate_ram_hash(&data);
        let ram_hash_time = ram_hash_start.elapsed().as_millis();
        println!("File hash from RAM: {} (completed in {} ms)", ram_hash, ram_hash_time);

        // Clean up
        fs::remove_file(temp_file)?;

        Ok(FileOperationResults {
            write_time,
            read_time,
            ram_load_time,
            disk_hash_time,
            ram_hash_time,
        })
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

