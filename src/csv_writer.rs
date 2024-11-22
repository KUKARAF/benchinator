use std::fs::File;
use std::io::{self, Write};

pub struct CsvWriter {
    file: File,
}

impl CsvWriter {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::create(file_path)?;
        Ok(CsvWriter { file })
    }

    pub fn write_row(&mut self, data: &[&str]) -> io::Result<()> {
        let row = data.join(",") + "\n";
        self.file.write_all(row.as_bytes())
    }
}

