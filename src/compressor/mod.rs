use std::time::{Instant};
use std::{fmt::Error, path::Path};

use crate::packager::pack;
use crate::packager::unpack;

mod compress;
mod decompress;

const MAX_ENTRY_COUNT: usize = 4097;
const INITIAL_CODE_WIDTH: u8 = 8;
const MAX_CODE_WIDTH: u8 = 12;

pub const EXTENSION: &str = "pressrs";

pub fn compress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    let now = Instant::now();
    let result = Ok(compress::lzw_compress(&pack(path)));

    println!(
        "Compression took {} ms",
        now.elapsed().as_millis()
    );
    return result;
}

pub fn decompress(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    let now = Instant::now();
    unpack(decompress::lzw_decompress(path), output);

     println!(
        "Decompression took {} ms",
        now.elapsed().as_millis()
    );
}