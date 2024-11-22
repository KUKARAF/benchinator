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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting benchmarks...");

    let file_ops = FileOperations::new();
    let git_ops = GitOperations::new();
    let docker_ops = DockerOperations::new();
    let download_ops = DownloadOperations::new();
    let build_run_ops = BuildRunOperations::new();
    let mut csv_writer = CsvWriter::new("benchmark_results.csv")?;

    // Implement benchmark tests
    let file_op_time = benchmark(|| file_ops.perform_operation().map_err(|e| e.to_string()))?;
    let git_op_time = benchmark(|| git_ops.perform_operation())?;
    docker_ops.perform_operation()?; // Docker system prune
    let docker_op_time = benchmark(|| docker_ops.perform_operation())?;
    let download_op_time = benchmark(|| async { download_ops.perform_operation().await }).await?;
    let build_run_op_time = benchmark(|| build_run_ops.perform_operation())?;

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

