# Press.rs Documentation

**Press.rs** is a high-performance Rust library for data packaging and compression using the LZW algorithm. It is designed to work seamlessly in both native system applications (CLI) and web environments via WebAssembly.

---

## 🛠 Compressor Module
Core LZW algorithm implementation.

- **`compress_from_path(path: impl AsRef<Path>) -> Result<Vec<u8>, Error>`**
    - **Description**: Compresses a byte slice using a variable-width (8-12 bit) LZW algorithm.
    - **Best for**: Compressing data from a file or folder on disk. 
- **`compress_raw(data: &[u8]) -> Vec<u8>`**
    - **Description**: Compresses a byte slice using a variable-width (8-12 bit) LZW algorithm.
    - **Best for**: Transforming raw data into a space-efficient bitstream.
- **`decompress_from_path_to_path(path: impl AsRef<Path>, output: impl AsRef<Path>)`**
    - **Description:** Decompresses archive and unpacks it into file system.
    - **Best for:** Extracting data from compressed archives and immediate writing it to disk.
- **`decompress_raw(data: &[u8]) -> Vec<u8>`**
    - **Description:** Decompress raw data without unpacking.
    - **Best for:** Extracting data from compressed archives.


## 📦 Packager Module
Utilities for archiving files and directories.

- **`pack(path: impl AsRef<Path>) -> Vec<u8>`**
    - **Description:** Scans a system path and serializes it into a packed binary buffer.
    - **System:** Uses standard filesystem access (`std::fs`).
- **`unpack(archive: Vec<u8>, path: impl AsRef<Path>)`**
    - **Description:** Takes an archive buffer and extracts it directly to the specified disk location.
    - **System:** Uses standard filesystem access (`std::fs`).
- **`pack_entries(entries: Vec<FileEntry>) -> Vec<u8>`**
    - **Description:** Packs a collection of in-memory `FileEntry` objects into a single binary buffer.
    - **WASM:** Primary method for web-based packaging without direct disk access.
- **`unpack_to_entries(archive: Vec<u8>) -> Vec<FileEntry>`**
    - **Description:** Parses a binary buffer and reconstructs it into a list of `FileEntry` objects in memory.
    - **WASM:** Primary method for web-based extraction where files are handled as blobs.

## Native Usage Example
```rust
use press_rs;

fn main() {
    // 1. Pack a directory and compress it
    let packed = press_rs::packager::pack("my_folder");
    let compressed = press_rs::compressor::compress_raw(&packed);
    
    // 2. Save the result to disk
    std::fs::write("backup.pressrs", compressed).unwrap();
}
```
## WASM Usage Example
```rust
use press_rs::packager::FileEntry;

// Data obtained from a browser file input
let entry = FileEntry {
    name: "web_upload.txt".into(),
    data: vec![10, 20, 30],
    is_dir: false,
};

let archive = press_rs::packager::pack_entries(vec![entry]);
let compressed = press_rs::compressor::compress_raw(&archive);
```