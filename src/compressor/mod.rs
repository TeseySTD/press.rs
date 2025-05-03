use std::{fmt::Error, path::Path};

use crate::packeger::pack;
use crate::packeger::unpack;

mod compress;
mod decompress;

pub const EXTENSION: &str = "pressrs";

pub fn compress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    return Ok(compress::compress(pack(path)));
}

pub fn decompress(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    unpack(decompress::decompress(path), output);
}
