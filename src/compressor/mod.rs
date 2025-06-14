use std::time::{Instant};
use std::{fmt::Error, path::Path};

use crate::packeger::pack;
use crate::packeger::unpack;

mod compress;
mod decompress;

const MAX_ENTRY_COUNT: usize = 4097;
const INITIAL_WIDTH: u8 = 8;
const MAX_CODE_WIDTH: u8 = 12;

pub const EXTENSION: &str = "pressrs";

pub fn compress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    let now = Instant::now();
    let result = Ok(compress::compress(&pack(path)));

    println!(
        "Compression took {} ms",
        now.elapsed().as_millis()
    );
    return result;
}

pub fn decompress(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    unpack(decompress::decompress(path), output);
}
