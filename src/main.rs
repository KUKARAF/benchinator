mod file_operations;
mod git_operations;
mod docker_operations;
mod csv_writer;

use file_operations::FileOperations;
use git_operations::GitOperations;
use docker_operations::DockerOperations;
use csv_writer::CsvWriter;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting benchmarks...");

    let file_ops = FileOperations::new();
    let git_ops = GitOperations::new();
    let docker_ops = DockerOperations::new();
    let mut csv_writer = CsvWriter::new("benchmark_results.csv");

    // Implement basic benchmark tests
    let file_op_time = benchmark(|| file_ops.perform_operation());
    let git_op_time = benchmark(|| git_ops.perform_operation());
    let docker_op_time = benchmark(|| docker_ops.perform_operation());

    // Write results to CSV
    csv_writer.write_row(&["Operation", "Time (ms)"])?;
    csv_writer.write_row(&["File Operation", &file_op_time.to_string()])?;
    csv_writer.write_row(&["Git Operation", &git_op_time.to_string()])?;
    csv_writer.write_row(&["Docker Operation", &docker_op_time.to_string()])?;

    println!("Benchmarks completed. Results written to benchmark_results.csv");
    Ok(())
}

fn benchmark<F>(f: F) -> u128
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed().as_millis()
}

