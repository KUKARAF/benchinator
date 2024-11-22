use reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    download: DownloadConfig,
}

#[derive(Deserialize)]
struct DownloadConfig {
    url: String,
    output: String,
}

pub struct DownloadOperations {
    config: Config,
}

impl DownloadOperations {
    pub fn new() -> Result<Self, String> {
        let config = Self::load_or_create_config()?;
        Ok(DownloadOperations { config })
    }

    fn load_or_create_config() -> Result<Config, String> {
        let config_path = Path::new("config.toml");
        if config_path.exists() {
            let config_str = std::fs::read_to_string(config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            toml::from_str(&config_str)
                .map_err(|e| format!("Failed to parse config file: {}", e))
        } else {
            let default_config = Config {
                download: DownloadConfig {
                    url: "https://testing.taxi/wp-content/uploads/2023/06/compressed-txt-100M.zip".to_string(),
                    output: "downloaded_file.zip".to_string(),
                },
            };
            let toml_str = toml::to_string(&default_config)
                .map_err(|e| format!("Failed to serialize default config: {}", e))?;
            std::fs::write(config_path, toml_str)
                .map_err(|e| format!("Failed to write default config file: {}", e))?;
            Ok(default_config)
        }
    }

    pub async fn perform_operation(&self) -> Result<(), String> {
        println!("Performing download operation...");
        
        let url = &self.config.download.url;
        let output = &self.config.download.output;

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
