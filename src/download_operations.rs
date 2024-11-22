use reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct DownloadOperations;

impl DownloadOperations {
    pub fn new() -> Self {
        DownloadOperations
    }

    pub async fn perform_operation(&self) -> Result<(), String> {
        println!("Performing download operation...");
        
        let url = "https://example.com/largefile.bin"; // Replace with an actual large file URL
        let output = "downloaded_file.bin";

        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Failed to GET from {}: {}", url, e))?;
        let mut file = File::create(output)
            .await
            .map_err(|e| format!("Failed to create file '{}': {}", output, e))?;

        let content = response.bytes()
            .await
            .map_err(|e| format!("Failed to get response bytes: {}", e))?;

        file.write_all(&content)
            .await
            .map_err(|e| format!("Failed to write content to file: {}", e))?;

        println!("File downloaded successfully.");
        Ok(())
    }
}
