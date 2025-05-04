use std::{
    fs,
    path::Path,
};
use header:: ENTRY_SIZE;
use pack::{pack_directory, pack_file};

mod header;
mod pack;
mod unpack;

pub fn pack(path: impl AsRef<Path>) -> Vec<u8> {
    let mut archive = Vec::<u8>::new();
    if path.as_ref().is_dir() {
        archive.extend(pack_directory(path.as_ref(), path.as_ref()));
    } else {
        archive.extend(pack_file(path.as_ref(), path.as_ref()));
    }

    // Add 2 empty entries to mark the end of the archive
    archive.extend(std::iter::repeat(0).take(ENTRY_SIZE * 2));

    return archive;
}

pub fn unpack(archive: Vec<u8>, path: impl AsRef<Path>) {
    if !path.as_ref().exists() {
        fs::create_dir_all(path.as_ref()).expect("Cannot create directory");
    }
    unpack::unpack(archive, path);
}
