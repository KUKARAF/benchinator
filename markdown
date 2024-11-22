name: Build Executables

on:
  push:
    branches: [ "master" ]
  pull_request:
    branches: [ "master" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Build Linux Binary
      run: cargo build --release
      
    - name: Upload Linux Artifact
      uses: actions/upload-artifact@v3
      with:
        name: system-benchmark-linux
        path: target/release/system_benchmark

  build-windows:
    runs-on: windows-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Build Windows Binary
      run: cargo build --release
      
    - name: Upload Windows Artifact
      uses: actions/upload-artifact@v3
      with:
        name: system-benchmark-windows
        path: target/release/system_benchmark.exe
