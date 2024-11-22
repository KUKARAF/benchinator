mod file_operations;
mod git_operations;
mod docker_operations;
mod csv_writer;
mod download_operations;
mod build_run_operations;

use file_operations::FileOperations;
use git_operations::GitOperations;
use docker_operations::DockerOperations;
use csv_writer::CsvWriter;
use download_operations::DownloadOperations;
use build_run_operations::BuildRunOperations;
use std::time::Instant;
use std::fs;
use std::path::Path;

fn ensure_config_and_artifacts() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure config.toml exists
    if !Path::new("config.toml").exists() {
        fs::write("config.toml", "# Configuration file\n[download]\nurl = \"https://example.com/file.zip\"\noutput = \"downloaded_file.zip\"\n")?;
        println!("Created config.toml with default settings.");
    }

    // Ensure artifacts folder exists
    if !Path::new("artifacts").exists() {
        fs::create_dir("artifacts")?;
        println!("Created artifacts folder.");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting benchmarks...");

    // Ensure config and artifacts exist
    ensure_config_and_artifacts()?;

    let file_ops = FileOperations::new();
    let git_ops = GitOperations::new();
    let docker_ops = DockerOperations::new();
    let download_ops = DownloadOperations::new().map_err(|e| e.to_string())?;
    let build_run_ops = BuildRunOperations::new();
    let mut csv_writer = CsvWriter::new("artifacts/benchmark_results.csv")?;

    // Implement benchmark tests
    let file_op_time = benchmark(|| async { file_ops.perform_operation().map_err(|e| e.to_string()) }).await?;
    let git_op_time = benchmark(|| async { git_ops.perform_operation() }).await?;
    docker_ops.perform_operation()?; // Docker system prune
    let docker_op_time = benchmark(|| async { docker_ops.perform_operation() }).await?;
    let download_op_time = benchmark(|| async { download_ops.perform_operation().await }).await?;
    let build_run_op_time = benchmark(|| async { build_run_ops.perform_operation() }).await?;

    // Write results to CSV
    csv_writer.write_row(&["Operation", "Time (ms)"])?;
    csv_writer.write_row(&["File Operation", &file_op_time.to_string()])?;
    csv_writer.write_row(&["Git Operation", &git_op_time.to_string()])?;
    csv_writer.write_row(&["Docker Operation", &docker_op_time.to_string()])?;
    csv_writer.write_row(&["Download Operation", &download_op_time.to_string()])?;
    csv_writer.write_row(&["Build and Run Operation", &build_run_op_time.to_string()])?;

    // Calculate average times
    let total_time = file_op_time + git_op_time + docker_op_time + download_op_time + build_run_op_time;
    let average_time = total_time / 5;
    csv_writer.write_row(&["Average Time", &average_time.to_string()])?;

    // Print results to console
    println!("File Operation: {} ms", file_op_time);
    println!("Git Operation: {} ms", git_op_time);
    println!("Docker Operation: {} ms", docker_op_time);
    println!("Download Operation: {} ms", download_op_time);
    println!("Build and Run Operation: {} ms", build_run_op_time);
    println!("Average Time: {} ms", average_time);

    println!("Benchmarks completed. Results written to benchmark_results.csv");
    Ok(())
}

async fn benchmark<F, Fut, T>(f: F) -> Result<u128, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, String>>,
{
    let start = Instant::now();
    f().await?;
    Ok(start.elapsed().as_millis())
}

