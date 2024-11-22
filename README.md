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
All settings can be configured through `config.toml` including:
- Download URLs
- Git file count
- Docker image settings
- Run type names

## Notes
- The tool automatically creates required directories
- Results are stored with timestamps
- Cleanup is performed after tests complete
