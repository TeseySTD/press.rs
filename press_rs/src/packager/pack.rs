use std::{fs, path::Path};

use crate::packager::FileEntry;

use super::header::{EntryType, Header, ENTRY_SIZE};

pub fn pack_from_file_entries(entries: Vec<FileEntry>) -> Vec<u8> {
    let mut stream = Vec::new();

    for entry in entries {
        let entry_type = if entry.is_dir {
            EntryType::Directory
        } else {
            EntryType::File
        };

        let header = Header::from_values(entry.name, entry.data.len(), entry_type);

        stream.extend(header.to_bytes());

        if !entry.is_dir {
            let data = file_as_entries(entry.data);
            stream.extend(data);
        }
    }

    // Add 2 empty entries to mark the end of the archive
    stream.extend(std::iter::repeat(0).take(ENTRY_SIZE * 2));
    stream
}

pub fn pack_directory(root: &Path, path: &Path) -> Vec<u8> {
    let mut stream = Vec::new();

    let rel = path.strip_prefix(root).expect("path must start with root");
    let rel_str = rel.to_string_lossy().replace('\\', "/");

    let header = Header::from_values(rel_str, 0, EntryType::Directory);

    stream.extend(header.to_bytes());

    for entry in fs::read_dir(path).expect("Cannot read directory") {
        let entry = entry.expect("Cannot read directory entry");
        let entry_path = entry.path();

        if entry.file_type().unwrap().is_dir() {
            stream.extend(pack_directory(root, &entry_path));
        } else {
            stream.extend(pack_file(root, &entry_path));
        }
    }

    stream
}

pub fn pack_file(root: &Path, path: &Path) -> Vec<u8> {
    let mut stream = Vec::new();
    let rel_str: String;

    if root == path {
        rel_str = path.file_name().unwrap().to_str().unwrap().to_string();
    } else {
        let rel = path.strip_prefix(root).expect("path must start with root");
        rel_str = rel.to_string_lossy().replace('\\', "/");
    }

    let header = Header::from_values(
        rel_str,
        path.metadata().unwrap().len() as usize,
        EntryType::File,
    );

    stream.extend(header.to_bytes());

    let mut data = fs::read(path).expect("Cannot read file");
    data = file_as_entries(data);
    stream.extend(data);

    stream
}

pub fn file_as_entries(mut file: Vec<u8>) -> Vec<u8> {
    let rem = file.len() % ENTRY_SIZE;
    if rem != 0 {
        let pad = ENTRY_SIZE - rem;
        file.extend(std::iter::repeat(0).take(pad));
    }
    file
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_pack_from_file_entries_logic() {
        let entries = vec![
            FileEntry {
                name: "test.txt".to_string(),
                data: b"hello binary".to_vec(),
                is_dir: false,
            },
            FileEntry {
                name: "folder".to_string(),
                data: vec![],
                is_dir: true,
            },
        ];

        let result = pack_from_file_entries(entries);
        assert!(result.len() >= ENTRY_SIZE * 4); // 2 headers + 1 file + 2 empty
    }

    #[test]
    fn test_pack_file_logic() {
        // Arrange
        let dir = tempdir().expect("Failed to create temp dir");
        let test_file = dir.path().join("unit_test.txt");
        let content = b"test content for unit packing";
        fs::write(&test_file, content).unwrap();

        // Act
        let result = pack_file(dir.path(), &test_file);

        // Assert
        assert!(!result.is_empty());
        assert!(result.len() >= crate::packager::header::ENTRY_SIZE + content.len());
    }

    #[test]
    fn test_pack_directory_recursive_logic() {
        // Arrange
        let dir = tempdir().expect("Failed to create temp dir");
        let sub = dir.path().join("nested");
        fs::create_dir_all(&sub).unwrap();
        fs::write(dir.path().join("1.txt"), "c1").unwrap();
        fs::write(sub.join("2.txt"), "c2").unwrap();

        // Act
        let result = pack_directory(dir.path(), dir.path());

        // Assert
        assert!(!result.is_empty());
        // Should contain entries for 2 files and potentially directories
        assert!(result.len() > crate::packager::header::ENTRY_SIZE * 2);

        fs::remove_dir_all(dir).ok();
    }
}
