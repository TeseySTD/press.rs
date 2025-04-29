use std::{fmt::Error, path::Path};

mod compress;
mod decompress;

pub fn compress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    todo!();
}

pub fn decompress(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    todo!();
}
