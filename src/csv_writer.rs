use std::fs::File;
use std::io::{self, Write};

use std::io::{self, Write, BufWriter};

pub struct CsvWriter {
    writer: BufWriter<File>,
}

impl CsvWriter {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);
        Ok(CsvWriter { writer })
    }

    pub fn write_row(&mut self, data: &[&str]) -> io::Result<()> {
        let row = data.join(",") + "\n";
        self.writer.write_all(row.as_bytes())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

