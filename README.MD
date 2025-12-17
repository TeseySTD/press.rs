<div align="center">
    <img src="./press_rs_logo.svg" alt="press.rs logo" width="175px">
    

**A lightweight, pure-Rust file compression and archiving utility.**

[![Rust](https://img.shields.io/badge/Rust-orange?logo=rust&style=flat-square)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](./LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/TeseySTD/press.rs/rust.yml?style=flat-square&label=Build)](https://github.com/TeseySTD/press.rs/actions)
[![Last Commit](https://img.shields.io/github/last-commit/TeseySTD/press.rs?style=flat-square&label=Last%20Commit)](https://github.com/TeseySTD/press.rs/commits/main)
</div>

## 📖 Overview

**PressRs** is a custom data compression tool built from scratch in Rust. It combines a **TAR-like packager** with an **LZW (Lempel–Ziv–Welch) compressor** to reduce file sizes while preserving directory structures.

It is designed as an educational project to demonstrate low-level bit manipulation, dictionary-based compression algorithms, and file stream handling in Rust.

## ✨ Features

- **Custom LZW Implementation**: Uses variable-width codes (9 to 12 bits) with dynamic dictionary resetting.
- **Archive Capability**: Bundles multiple files and directories into a single `.pressrs` file.
- **Memory Efficient**: Streams data using buffered readers/writers to handle large files.
- **CLI**: Simple text-based interface for selecting modes.

## 🛠️ Technical Details

### Compression Algorithm (LZW)
The core of PressRs is the LZW algorithm.
1.  **Dictionary**: Starts with a default ASCII set (0-255).
2.  **Dynamic Growth**: As patterns are found, new codes are added to the dictionary.
3.  **Variable Bit Width**: The output code size starts at **9 bits** and grows up to **12 bits** as the dictionary fills up.
4.  **Reset Mechanism**: Once the dictionary reaches its limit (4096 entries), it sends a `Clear Code` and resets, preventing memory overflow and adapting to new data patterns.

### Packaging Format
PressRs uses a custom binary format similar to TAR:
- It traverses the target directory recursively.
- Each file is preceded by a metadata header (containing relative path and size).
- The continuous stream of file data is then passed to the LZW compressor.

## 🚀 Usage

### Installation
Ensure you have Rust installed, then clone and build:

```bash
git clone https://github.com/TeseySTD/press.rs.git
cd press.rs 
cargo build --release
```

Executable will be at `./target/release/press_rs`