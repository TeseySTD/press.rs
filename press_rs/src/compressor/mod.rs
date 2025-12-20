use std::time::Instant;
use std::{fmt::Error, path::Path};

use crate::packager::pack;
use crate::packager::unpack;

mod compress;
mod decompress;

/// The maximum number of entries in the LZW dictionary (4096 + 1).
/// Corresponds to a 12-bit code width (2^12 = 4096).
const MAX_ENTRY_COUNT: usize = 4097;

/// The initial bit width for LZW codes.
const INITIAL_CODE_WIDTH: u8 = 8;

/// The maximum bit width for LZW codes.
const MAX_CODE_WIDTH: u8 = 12;

/// The default file extension for archives created by this crate.
pub const EXTENSION: &str = "pressrs";

/// Compresses a file or directory path into a packed archive.
///
/// This function performs two steps:
/// 1. **Packing:** Traverses the directory (or single file) and serializes it into a binary format.
/// 2. **Compression:** Applies LZW compression to the packed data.
///
/// # Arguments
///
/// * `path` - A path to the file or directory to compress.
///
/// # Returns
///
/// Returns a `Result` containing the compressed bytes (`Vec<u8>`) or an error.
///
/// # Side Effects
///
/// * Prints the duration of the compression process to stdout.
///
/// # Examples
///
/// ```no_run
/// use press_rs::compressor::compress_from_path;
///
/// let compressed_data = compress_from_path("./my_folder").expect("Compression failed");
/// // Now you can save `compressed_data` to a file, e.g., "archive.pressrs"
/// ```
pub fn compress_from_path(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    let now = Instant::now();
    let result = Ok(compress::lzw_compress(&pack(path)));

    println!("Compression took {} ms", now.elapsed().as_millis());
    return result;
}

/// Compresses raw byte data using LZW algorithm without packing.
///
/// Use this function if you already have binary data (e.g., a serialized struct or text)
/// and just want to compress it without creating a file archive structure 
/// or you already have packed data.
///
/// # Arguments
///
/// * `data` - A slice of bytes to compress.
///
/// # Examples
///
/// ``` no_run
/// use press_rs::compressor::compress_raw;
/// use press_rs::packager::pack;
/// 
/// let data = pack("./my_folder");
/// let compressed = compress_raw(&data);
/// ```
pub fn compress_raw(data: &[u8]) -> Vec<u8> {
    compress::lzw_compress(data)
}

/// Decompresses an LZW archive from a file and unpacks it to a destination.
///
/// This function performs the reverse of [`compress_from_path`]:
/// 1. **Decompression:** Reads the file at `path` and decodes LZW.
/// 2. **Unpacking:** Recreates the original file/directory structure at `output`.
///
/// # Arguments
///
/// * `path` - Path to the `.pressrs` archive file.
/// * `output` - Path to the directory where files should be extracted.
///
/// # Side Effects
///
/// * Creates files and directories on the disk.
/// * Prints the duration of the decompression process to stdout.
///
/// # Examples
///
/// ```no_run
/// use press_rs::compressor::decompress_from_path_to_path;
///
/// // Extracts contents of "backup.pressrs" into the "restored" folder
/// decompress_from_path_to_path("backup.pressrs", "./restored");
/// ```
pub fn decompress_from_path_to_path(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    let now = Instant::now();
    unpack(decompress::lzw_decompress(path), output);

    println!("Decompression took {} ms", now.elapsed().as_millis());
}

/// Decompresses raw LZW-encoded bytes.
///
/// Use this to restore data compressed with [`compress_raw`].
///
/// # Arguments
///
/// * `data` - The compressed byte slice.
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the original uncompressed data.
/// 
/// # Examples
/// 
/// ``` no_run
/// use press_rs::compressor::{decompress_raw, compress_raw};
/// use press_rs::packager::pack;
/// 
/// let data = pack("./my_folder");
/// let compressed = compress_raw(&data);
/// let decompressed = decompress_raw(compressed.as_slice());
/// ```
pub fn decompress_raw(data: &[u8]) -> Vec<u8> {
    decompress::lzw_decompress_bytes(data)
}