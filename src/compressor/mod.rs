use std::{fmt::Error, path::Path};

use crate::packeger::{pack};

mod compress;
mod decompress;

pub const EXTENSION: &str = "pressrs";

pub fn compress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    return Ok(compress::compress(pack(path)));
}

#[allow(dead_code, unused_variables)]
pub fn decompress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    todo!();
}
