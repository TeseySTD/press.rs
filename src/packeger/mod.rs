use std::{
    fs,
    path::Path,
};

use header::{Header, ENTRY_SIZE};

mod header;

pub fn pack(path: impl AsRef<Path>) -> Vec<u8> {
    let mut archive = Vec::<u8>::new();
    if path.as_ref().is_dir() {
        archive.extend(pack_directory(path));
    } else {
        archive.extend(pack_file(path));
    }

    archive.extend(std::iter::repeat(2).take(ENTRY_SIZE));

    return archive;
}

fn pack_directory(path: impl AsRef<Path>) -> Vec<u8> {
    let mut stream = Vec::<u8>::new();

    let mut header = Header::new();
    header.set_name(
        path.as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );
    header.set_size(0);
    header.set_typeflag(header::EntryType::Directory);

    stream.extend(header.as_bytes());

    let dir_entries = fs::read_dir(path)
        .expect("Cannot read directory")
        .map(|entry| entry.expect("Cannot read directory entry"));

    for dir_entry in dir_entries {
        if dir_entry
            .file_type()
            .expect("Cannot get file type")
            .is_dir()
        {
            stream.extend(pack_directory(dir_entry.path()));
        } else {
            stream.extend(pack_file(dir_entry.path()));
        }
    }

    return stream;
}

fn pack_file(path: impl AsRef<Path>) -> Vec<u8> {
    let mut stream = Vec::<u8>::new();

    let mut header = Header::new();
    header.set_name(
        path.as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );
    header.set_size(path.as_ref().metadata().unwrap().len() as usize);
    header.set_typeflag(header::EntryType::File);

    stream.extend(header.as_bytes());

    let file = fs::read(path).expect("Cannot read file");
    stream.extend(file_as_entries(file));

    return stream;
}

fn file_as_entries(mut file: Vec<u8>) -> Vec<u8> {
    let rem = file.len() % ENTRY_SIZE;
    if rem != 0 {
        let pad = ENTRY_SIZE - rem;
        file.extend(std::iter::repeat(0).take(pad));
    }
    file
}