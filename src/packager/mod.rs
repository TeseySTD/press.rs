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
    use std::path::Path;
    #[test]
    fn test_pack_unpack_integration_flow() {
        // Arrange
        let root = "test_pack_env";
        let src_dir = format!("{}/src", root);
        let dst_dir = format!("{}/dst", root);
        let file_name = "hello.txt";
        let content = "Hello Integration Test";

        fs::create_dir_all(&src_dir).expect("Failed to create src dir");
        fs::write(format!("{}/{}", src_dir, file_name), content).expect("Failed to write file");

        let archive_data = pack(Path::new(&src_dir));

        // Act
        unpack(archive_data, Path::new(&dst_dir));

        // Assert
        let unpacked_file_path = format!("{}/{}", dst_dir, file_name);
        assert!(
            Path::new(&unpacked_file_path).exists(),
            "Unpacked file should exist"
        );

        let unpacked_content =
            fs::read_to_string(&unpacked_file_path).expect("Failed to read unpacked file");
        assert_eq!(unpacked_content, content);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn test_pack_non_existent_path() {
        // Arrange
        let path = Path::new("non_existent_folder");

        let result = std::panic::catch_unwind(|| pack(path));

        // Assert
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
