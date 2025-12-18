use std::{fs, path::Path};

use super::header::{ENTRY_SIZE, EntryType, Header};

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
    use std::path::Path;

    #[test]
    fn test_pack_file_logic() {
        // Arrange
        // Create a temporary directory to ensure a clear root/file hierarchy
        let test_root = Path::new("test_pack_file_dir");
        let test_file = test_root.join("unit_test_pack_file.txt");
        let content = b"test content for unit packing";

        fs::create_dir_all(test_root).expect("Failed to create test root");
        fs::write(&test_file, content).expect("Failed to write test file");

        // Act
        let result = pack_file(&test_root, &test_file);

        // Assert
        assert!(!result.is_empty());
        // Header (169 bytes) + content
        assert!(result.len() >= crate::packager::header::ENTRY_SIZE + content.len());

        // Cleanup
        fs::remove_dir_all(test_root).ok();
    }
    #[test]
    fn test_pack_directory_recursive_logic() {
        // Arrange
        let root = "unit_test_pack_dir";
        let sub = format!("{}/nested", root);
        fs::create_dir_all(&sub).unwrap();
        fs::write(format!("{}/1.txt", root), "c1").unwrap();
        fs::write(format!("{}/2.txt", sub), "c2").unwrap();

        // Act
        let result = pack_directory(Path::new(root), Path::new(root));

        // Assert
        assert!(!result.is_empty());
        // Should contain entries for 2 files and potentially directories
        assert!(result.len() > crate::packager::header::ENTRY_SIZE * 2);

        fs::remove_dir_all(root).ok();
    }
}
