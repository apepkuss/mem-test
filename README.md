
# Memory Consumption Test of WasmEdge Runtime

Inspired by [Issue 1805: Potential Memory Leak Issue](https://github.com/WasmEdge/WasmEdge/issues/1805), this repository is to measure the memory consumption of WasmEdge runtime with Rust and Golang SDK.

## Test Setup

The test is performed in the following software (and hardware) environments:

- OS and Architecture
  - Ubuntu 22.04 (WSL2, x86_64)
  - Ubuntu 20.04 (Docker v4.17.0, Apple M1)
  - Fedora 37 (Docker v4.17.0, Apple M1)
  - macOS 13.2.1 (Apple M1)

- WasmEdge-0.12.0-alpha.2-38-gdad018af

- WasmEdge Rust SDK (wasmedge-sdk-0.8.0-rc)

- WasmEdge Go SDK v0.12.0-alpha.2

- Test Code
  - Rust version: ./mem-test/src/main.rs
  - Golang version: ./hello-go/test.go

The metrics used in the test are as follows:

- Memory Consumption Percentage (MCP)
- Resident Set Size (RSS)
  
## Results

- Memory Consumption Percentage (MCP)

  - Test with Rust SDK
    Ubuntu-22.04 on WSL2 + x86 | Ubuntu-20.04 on Docker + Apple M1
    :-------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/mem-ubuntu2204-wsl2-x86-rs.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/mem-ubuntu2004-docker-m1-rs.png)
    
    Fedora 37 on Docker + Apple M1  |  macOS 13.2.1 + Apple M1
    :------------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/mem-fedora37-docker-m1-rs.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/mem-macos-m1-rs.png)

  - Test with Go SDK
    Ubuntu-22.04 on WSL2 + x86 | Ubuntu-20.04 on Docker + Apple M1
    :-------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/mem-ubuntu2204-wsl2-x86-go.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/mem-ubuntu2004-docker-m1-go.png)
    
    Fedora 37 on Docker + Apple M1  |  macOS 13.2.1 + Apple M1
    :------------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/mem-fedora37-docker-m1-go.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/mem-macos-m1-go.png)

- Resident Set Size (RSS)

  - Test with Rust SDK
    Ubuntu-22.04 on WSL2 + x86 | Ubuntu-20.04 on Docker + Apple M1
    :-------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/rss-ubuntu2204-wsl2-x86-rs.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/rss-ubuntu2004-docker-m1-rs.png)
    
    Fedora 37 on Docker + Apple M1  |  macOS 13.2.1 + Apple M1
    :------------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/rss-fedora37-docker-m1-rs.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/rss-macos-m1-rs.png)
    
  - Test with Go SDK
    Ubuntu-22.04 on WSL2 + x86 | Ubuntu-20.04 on Docker + Apple M1
    :-------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/rss-ubuntu2204-wsl2-x86-go.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/rss-ubuntu2004-docker-m1-go.png)
    
    Fedora 37 on Docker + Apple M1  |  macOS 13.2.1 + Apple M1
    :------------------------------:|:-------------------------:
    ![](images/0.12.0-alpha.2-38-gdad018af/rss-fedora37-docker-m1-go.png)  |  ![](images/0.12.0-alpha.2-38-gdad018af/rss-macos-m1-go.png)
