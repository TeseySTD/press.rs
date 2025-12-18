use header::ENTRY_SIZE;
use pack::{pack_directory, pack_file};
use std::{fs, path::Path};

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_pack_unpack_integration_flow() {
        // Arrange
        let dir = tempdir().expect("Failed to create temp dir");
        let src_dir = dir.path().join("src");
        let dst_dir = dir.path().join("dst");
        let file_name = "hello.txt";
        let content = "Hello Tempfile Integration Test Content";

        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join(file_name), content).unwrap();

        // Act
        let archive_data = pack(&src_dir);
        unpack(archive_data, &dst_dir);

        // Assert
        let unpacked_file_path = dst_dir.join(file_name);
        assert!(unpacked_file_path.exists(), "Unpacked file should exist");

        let unpacked_content = fs::read_to_string(unpacked_file_path).unwrap();
        assert_eq!(unpacked_content, content, "Content must match original");
    }

    #[test]
    fn test_pack_non_existent_path() {
        // Arrange
        let dir = tempdir().expect("Failed to create temp dir");
        let non_existent = dir.path().join("ghost_folder");

        // Act
        let result = std::panic::catch_unwind(|| pack(&non_existent));

        // Assert
        assert!(
            result.is_err() || result.unwrap().is_empty(),
            "Should handle non-existent path gracefully"
        );
    }
}
