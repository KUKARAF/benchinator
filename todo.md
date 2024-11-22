A system benchmarking tool built in Rust that measures and compares various system operations including file I/O, memory operations, git operations, and application startup times.

Core Features:
- File Operations:
  * Write/Read 500MB random file
  * Load/Read 500MB to/from RAM
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
  * Export results to CSV
  * Calculate average times per operation type
  * Track individual test times

Note: Since this is a command-line tool running directly on the system, the UI will focus on clear, formatted console output with:
- Progress indicators for long-running operations
- Color-coded success/failure states
- Clean tabular output for results display

Important: This tool requires Git to be pre-installed on the system. Since Docker operations cannot be performed on Replit, those specific benchmarks should be made optional or replaced with equivalent operations that can run in the Replit environment.
