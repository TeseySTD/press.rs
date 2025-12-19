use std::time::Instant;
use std::{fmt::Error, path::Path};

use crate::packager::pack;
use crate::packager::unpack;

mod compress;
mod decompress;

const MAX_ENTRY_COUNT: usize = 4097;
const INITIAL_CODE_WIDTH: u8 = 8;
const MAX_CODE_WIDTH: u8 = 12;

pub const EXTENSION: &str = "pressrs";

/// Compresses a path and packs it.
pub fn compress_from_path(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    let now = Instant::now();
    let result = Ok(compress::lzw_compress(&pack(path)));

    println!("Compression took {} ms", now.elapsed().as_millis());
    return result;
}

/// Compresses raw data without packing.
pub fn compress_raw(data: &[u8]) -> Vec<u8> {
    compress::lzw_compress(data)
}

/// Decompresses a path and unpacks it.
pub fn decompress_from_path_to_path(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    let now = Instant::now();
    unpack(decompress::lzw_decompress(path), output);

    println!("Decompression took {} ms", now.elapsed().as_millis());
}

// Decompress raw data without unpacking.
pub fn decompress_raw(data: &[u8]) -> Vec<u8> {
    decompress::lzw_decompress_bytes(data)
}
