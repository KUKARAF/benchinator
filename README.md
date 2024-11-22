# System Benchmarking Tool

A comprehensive system benchmarking tool built in Rust that measures and compares various system operations.

## Features

### File Operations ✓
- Write/Read operations with 500MB random file
- Load/Read 500MB to/from RAM
- File hash calculation (both from disk and RAM)
- Configurable file sizes

### Git Operations ✓
- Repository initialization
- Random file generation and commits
- Configurable number of test files (default: 50)

### Download Operations ✓
- Configurable URL-based file downloads
- Progress tracking
- Error handling

### Build & Run Operations ✓
- Python Django application startup
- Docker system prune between operations
- Docker image pulling and testing

### Results Processing ✓
- CSV export of all benchmark results
- Average time calculations per operation
- Individual test time tracking
- Organized results storage in runs directory
- Run type selection for different test scenarios

## Prerequisites
- Git must be installed
- Docker must be installed
- Rust toolchain
- Python (for Django tests)

## Configuration
The tool uses a `config.toml` file for all configuration settings. This file is automatically generated with default values if not present.

### Available Configuration Options:

#### Download Settings
```toml
[download]
url = "https://testing.taxi/wp-content/uploads/2023/06/compressed-txt-100M.zip"
output = "downloaded_file.zip"
```
- `url`: The URL to download files from during benchmark tests
- `output`: Local filename for the downloaded file

#### Git Settings
```toml
[git]
files_count = 50
```
- `files_count`: Number of random files to generate and commit (default: 50)

#### Docker Settings
```toml
[docker]
image = "af2.corpo.t-mobile.pl/cindy-base-images/python:3.9.7-slim-buster"
test_command = ["python", "--version"]
```
- `image`: Docker image to use for container tests
- `test_command`: Command to run inside the container for testing

#### Run Type Settings
```toml
[runs]
names = ["security_off", "security_on"]
```
- `names`: List of run types that will be presented as options when saving results
- You can add as many run types as needed for different test scenarios

### Notes:
- The config file is generated automatically on first run if not present
- All settings can be modified to suit your specific testing needs
- Run types allow for organizing test results into different categories (e.g., with/without security features)
- Results are saved with timestamps and selected run type name for easy identification

## How to Run

1. Download the executable file
2. Run the executable:
```bash
./system_benchmark
```

On first run, the tool will:
- Create a default `config.toml` file if not present
- Create necessary directories (artifacts, runs)
- Run all benchmarks
- Prompt you to select a run type
- Save results to the `runs` directory

### Configuration
You can edit the `config.toml` file (created after first run) to customize:
- Download URL and output filename
- Number of git test files
- Docker image and test commands
- Run type names

### Results
- All benchmark results are saved in the `runs` directory
- Files are named with timestamp and selected run type: `YYYYMMDD_HHMMSS_runtype.csv`
- Each CSV file contains detailed timing for all operations

## Notes
- The tool automatically creates required directories
- Results are stored with timestamps
- Cleanup is performed after tests complete