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
        fs::write("config.toml", 
            "# Configuration file\n\
            [download]\n\
            url = \"https://testing.taxi/wp-content/uploads/2023/06/compressed-txt-100M.zip\"\n\
            output = \"downloaded_file.zip\"\n\
            \n\
            [git]\n\
            files_count = 50\n\
            \n\
            [docker]\n\
            image = \"af2.corpo.t-mobile.pl/cindy-base-images/python:3.9.7-slim-buster\"\n\
            test_command = [\"python\", \"--version\"]\n")?;
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
    let git_ops = GitOperations::new().map_err(|e| e.to_string())?;
    let docker_ops = DockerOperations::new().map_err(|e| e.to_string())?;
    let download_ops = DownloadOperations::new().map_err(|e| e.to_string())?;
    let build_run_ops = BuildRunOperations::new();
    let mut csv_writer = CsvWriter::new("artifacts/benchmark_results.csv")?;

    // Write header to CSV
    println!("Writing CSV header...");
    csv_writer.write_row(&["Operation", "Time (ms)"])?;
    csv_writer.flush()?;

    // Implement benchmark tests and write results immediately
    let file_op_results = file_ops.perform_operation()?;
    println!("Writing File Operation results...");
    csv_writer.write_row(&["File Write Operation", &file_op_results.write_time.to_string()])?;
    csv_writer.write_row(&["File Read Operation", &file_op_results.read_time.to_string()])?;
    csv_writer.write_row(&["RAM Load Operation", &file_op_results.ram_load_time.to_string()])?;
    csv_writer.write_row(&["Disk Hash Operation", &file_op_results.disk_hash_time.to_string()])?;
    csv_writer.write_row(&["RAM Hash Operation", &file_op_results.ram_hash_time.to_string()])?;
    csv_writer.flush()?;

    let total_file_time = file_op_results.write_time + file_op_results.read_time + 
                         file_op_results.ram_load_time + file_op_results.disk_hash_time + 
                         file_op_results.ram_hash_time;

    let git_op_time = benchmark(|| async { git_ops.perform_operation() }).await?;
    println!("Writing Git Operation result...");
    csv_writer.write_row(&["Git Operation", &git_op_time.to_string()])?;
    csv_writer.flush()?;

    docker_ops.perform_operation()?; // Docker system prune
    let docker_op_time = benchmark(|| async { docker_ops.perform_operation() }).await?;
    println!("Writing Docker Operation result...");
    csv_writer.write_row(&["Docker Operation", &docker_op_time.to_string()])?;
    csv_writer.flush()?;

    let download_op_time = benchmark(|| async { download_ops.perform_operation().await }).await?;
    println!("Writing Download Operation result...");
    csv_writer.write_row(&["Download Operation", &download_op_time.to_string()])?;
    csv_writer.flush()?;

    let build_run_op_time = benchmark(|| async { build_run_ops.perform_operation() }).await?;
    println!("Writing Build and Run Operation result...");
    csv_writer.write_row(&["Build and Run Operation", &build_run_op_time.to_string()])?;
    csv_writer.flush()?;

    // Calculate average times
    let total_time = total_file_time + git_op_time + docker_op_time + download_op_time + build_run_op_time;
    let average_time = total_time / 9; // Now counting all 5 file operations + 4 other operations
    println!("Writing Average Time result...");
    csv_writer.write_row(&["Average Time", &average_time.to_string()])?;
    csv_writer.flush()?;

    // Ensure all data is written to the file
    csv_writer.flush()?;

    // Print results to console
    println!("File Operations:");
    println!("  Write: {} ms", file_op_results.write_time);
    println!("  Read: {} ms", file_op_results.read_time);
    println!("  RAM Load: {} ms", file_op_results.ram_load_time);
    println!("  Disk Hash: {} ms", file_op_results.disk_hash_time);
    println!("  RAM Hash: {} ms", file_op_results.ram_hash_time);
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

