use std::path::Path;

pub fn decompress(path: impl AsRef<Path>) -> Vec<u8> {
    // TODO
    return std::fs::read(path).expect("Cannot read file");
}