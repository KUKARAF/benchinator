mod build_run_operations;
mod csv_writer;
mod docker_operations;
mod download_operations;
mod file_operations;
mod git_operations;

use build_run_operations::BuildRunOperations;
use charts::{Chart, Color, ScaleBand, ScaleLinear, VerticalBarView};
use chrono::Local;
use csv_writer::CsvWriter;
use docker_operations::DockerOperations;
use download_operations::DownloadOperations;
use file_operations::FileOperations;
use git_operations::GitOperations;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;
use toml::Value;

fn ensure_config_and_directories() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure config.toml exists
    if !Path::new("config.toml").exists() {
        fs::write(
            "config.toml",
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
            test_command = [\"python\", \"--version\"]\n\
            \n\
            [runs]\n\
            names = [\"security_off\", \"security_on\"]\n",
        )?;
        println!("Created config.toml with default settings.");
    }

    // Ensure artifacts and runs folders exist
    for dir in &["artifacts", "runs"] {
        if !Path::new(dir).exists() {
            fs::create_dir(dir)?;
            println!("Created {} folder.", dir);
        }
    }

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting benchmarks...");

    // Ensure config, artifacts, and runs directories exist
    ensure_config_and_directories()?;

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
    csv_writer.write_row(&[
        "File Write Operation",
        &file_op_results.write_time.to_string(),
    ])?;
    csv_writer.write_row(&[
        "File Read Operation",
        &file_op_results.read_time.to_string(),
    ])?;
    csv_writer.write_row(&[
        "RAM Load Operation",
        &file_op_results.ram_load_time.to_string(),
    ])?;
    csv_writer.write_row(&[
        "Disk Hash Operation",
        &file_op_results.disk_hash_time.to_string(),
    ])?;
    csv_writer.write_row(&[
        "RAM Hash Operation",
        &file_op_results.ram_hash_time.to_string(),
    ])?;
    csv_writer.flush()?;

    let total_file_time = file_op_results.write_time
        + file_op_results.read_time
        + file_op_results.ram_load_time
        + file_op_results.disk_hash_time
        + file_op_results.ram_hash_time;

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
    let total_time =
        total_file_time + git_op_time + docker_op_time + download_op_time + build_run_op_time;
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

    println!("Benchmarks completed. Moving results to runs directory...");

    // Read config to get run names
    let config_str = fs::read_to_string("config.toml")?;
    let config: Value = toml::from_str(&config_str)
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    let run_names = config
        .get("runs")
        .and_then(|r| r.get("names"))
        .and_then(|n| n.as_array())
        .ok_or_else(|| "Run names not found in config")?;

    // Convert run names to Vec<String> for dialog
    let run_options: Vec<String> = run_names
        .iter()
        .filter_map(|v| v.as_str())
        .map(String::from)
        .collect();

    // Show run options in terminal
    println!("\nSelect a run type by entering its number:");
    for (i, name) in run_options.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }
    print!("\nEnter selection (1-{}): ", run_options.len());
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let selection = input
        .trim()
        .parse::<usize>()
        .map_err(|_| "Invalid selection")?
        .checked_sub(1)
        .ok_or("Invalid selection")?;

    let run_name = run_options.get(selection).ok_or("Invalid selection")?;

    // Move results file to runs directory with timestamp and run name
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let new_filename = format!("runs/{}_{}.csv", timestamp, run_name);
    fs::rename("artifacts/benchmark_results.csv", &new_filename)?;

    // Cleanup
    println!("Cleaning up...");
    if Path::new("artifacts").exists() {
        fs::remove_dir_all("artifacts")?;
        println!("Artifacts directory cleaned up.");
    }

    println!("Results written to {}", new_filename);

    // Calculate and update averages for this run type
    update_run_type_averages(run_name)?;

    // Generate and save the stacked bar chart
    generate_bar_chart(run_name)?;

    Ok(())
}

// Function to update averages for a specific run type
fn update_run_type_averages(run_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    let runs_dir = Path::new("runs");
    let mut operation_totals: HashMap<String, (f32, u32)> = HashMap::new(); // (sum, count)

    // Read all CSV files for this run type
    for entry in fs::read_dir(runs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && path.extension().map_or(false, |ext| ext == "csv")
            && path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains(run_type)
        {
            let content = fs::read_to_string(&path)?;
            for line in content.lines().skip(1) {
                // Skip header
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 {
                    let operation = parts[0].trim().to_string();
                    if let Ok(time) = parts[1].trim().parse::<f32>() {
                        let (sum, count) = operation_totals.entry(operation).or_insert((0.0, 0));
                        *sum += time;
                        *count += 1;
                    }
                }
            }
        }
    }

    // Calculate averages and write to avg_<run_type>.csv
    let avg_file_path = format!("avg_{}.csv", run_type);
    let mut csv_writer = CsvWriter::new(&avg_file_path)?;

    // Write header
    csv_writer.write_row(&["Operation", "Average Time (ms)"])?;

    // Write averages and calculate total
    let mut total_average = 0.0f32;
    for (operation, (sum, count)) in operation_totals {
        let average = if count > 0 { sum / count as f32 } else { 0.0 };
        csv_writer.write_row(&[&operation, &average.to_string()])?;
        total_average += average;
    }

    // Write total in all caps
    csv_writer.write_row(&["TOTAL", &total_average.to_string()])?;
    csv_writer.flush()?;
    println!("Updated averages written to {}", avg_file_path);

    Ok(())
}

fn generate_bar_chart(run_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    let run_types = ["security_off", "security_on"];
    let mut operations: Vec<String> = Vec::new();
    let mut run_data: HashMap<String, Vec<f32>> = HashMap::new();

    // Read data from avg_*.csv files
    for &rt in &run_types {
        let avg_file = format!("avg_{}.csv", rt);
        if let Ok(content) = fs::read_to_string(&avg_file) {
            let mut times = Vec::new();
            for line in content.lines().skip(1) {  // Skip header
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 {
                    let operation = parts[0].trim();
                    if operation != "TOTAL" {  // Skip total row
                        if let Ok(time) = parts[1].trim().parse::<f32>() {
                            if !operations.contains(&operation.to_string()) {
                                operations.push(operation.to_string());
                            }
                            times.push(time);
                        }
                    }
                }
            }
            run_data.insert(rt.to_string(), times);
        }
    }

    // Prepare data for chart
    let mut chart_data = Vec::new();
    for op in &operations {
        if let (Some(off_times), Some(on_times)) = (run_data.get("security_off"), run_data.get("security_on")) {
            if let (Some(&off_time), Some(&on_time)) = (
                off_times.get(operations.iter().position(|x| x == op).unwrap()),
                on_times.get(operations.iter().position(|x| x == op).unwrap())
            ) {
                chart_data.push((op.as_str(), off_time));
                chart_data.push((op.as_str(), on_time));
            }
        }
    }

    // Create scales
    let x = ScaleBand::new()
        .set_domain(operations.iter().map(|s| s.to_string()).collect())
        .set_range(vec![0, 800 - 60 - 40])
        .set_inner_padding(0.2)
        .set_outer_padding(0.1);

    let max_time = run_data.values()
        .flat_map(|v| v.iter())
        .fold(0.0f32, |a, &b| a.max(b));

    let y = ScaleLinear::new()
        .set_domain(vec![0.0, max_time])
        .set_range(vec![600 - 90 - 50, 0]);

    // Create bar view with colors
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_colors(vec![
            Color::new_rgb(65, 105, 225),  // Royal Blue
            Color::new_rgb(220, 20, 60),   // Crimson Red
        ])
        .load_data(&chart_data)?;

    // Generate chart
    let chart_path = "runs/benchmark_comparison.svg";
    let mut chart = Chart::new()
        .set_width(800)
        .set_height(600)
        .set_margins(90, 40, 50, 60);

    chart.add_title("Benchmark Comparison: Security Off vs On".to_string())
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Time (ms)")
        .add_bottom_axis_label("Operations")
        .save(chart_path)?;

    println!("Comparison chart saved to {}", chart_path);
    Ok(())
}
