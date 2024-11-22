A system benchmarking tool built in Rust that measures and compares various system operations including file I/O, memory operations, git operations, and application startup times.

Core Features:
- File Operations:
  * Write/Read 500MB(default) random file
  * Load/Read 500MB(default) to/from RAM
  * Calculate file hash (both from disk and RAM)
- Git Operations:
  * Repository initialization and commit
  * Add and commit 50 randomly generated text files
- Download Operations:
  * File download from configurable URL
- Build and Run Operations:
  * Build and run a C project
  * Start a Python Django application
  * Include Docker system prune between operations
- Results Processing:
  * Export all results to CSV
  * Calculate average times per operation type
  * Track individual test times

Note: Since this is a command-line tool running directly on the system, the UI will focus on clear, formatted console output with:
- Clean tabular output for results display

Important: This tool requires Git and Docker to be pre-installed on the system. If not present then these particular tests should be skipped with a warning
