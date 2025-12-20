use header::ENTRY_SIZE;
use pack::{pack_directory, pack_file};
use std::{fs, path::Path};

use crate::packager::{pack::pack_from_file_entries, unpack::unpack_to_file_entries};

mod header;
mod pack;
mod unpack;

/// Represents a single entry (file or directory) within the archive.
///
/// This struct is primarily used when working with in-memory archives via [`pack_entries`]
/// or when inspecting archive contents via [`unpack_to_entries`].
///
/// # Examples
///
/// Creating a file entry manually:
/// ```
/// use press_rs::packager::FileEntry;
///
/// let file = FileEntry {
///     name: "documents/notes.txt".to_string(),
///     data: b"Remember to buy milk".to_vec(),
///     is_dir: false,
/// };
/// ```
///
/// Creating a directory entry:
/// ```
/// use press_rs::packager::FileEntry;
///
/// let dir = FileEntry {
///     name: "documents/".to_string(),
///     data: Vec::new(), // Directories have no content
///     is_dir: true,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FileEntry {
    /// The relative path of the file or directory within the archive.
    ///
    /// # Best Practices
    /// * Use forward slashes (`/`) as separators for better cross-platform compatibility.
    /// * Avoid absolute paths (e.g., `C:\Windows` or `/usr/bin`).
    pub name: String,

    /// The raw binary content of the file.
    ///
    /// If [`is_dir`](Self::is_dir) is `true`, this vector should generally be empty.
    pub data: Vec<u8>,

    /// Indicates whether this entry represents a directory.
    ///
    /// * If `true`, the entry is treated as a folder structure.
    /// * If `false`, the entry is treated as a regular file containing [`data`](Self::data).
    pub is_dir: bool,
}

/// Packs a file or directory into a binary archive
///
/// # Arguments
///
/// * `path` - The path to the file or directory to pack.
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the packed archive data.
///
/// # Examples
///
/// ```no_run
/// use press_rs::packager::pack;
///
/// // Pack a directory
/// let archive_bytes = pack("./src");
///
/// // Or pack a single file
/// let file_archive = pack("Cargo.toml");
/// ```
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

/// Packs a list of file entries into a binary archive. Useful for non-filesystem use.
///
/// # Arguments
///
/// * `entries` - A vector of [`FileEntry`]s to pack.
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the packed archive data.
///
/// # Examples
///
/// ```
/// use press_rs::packager::{pack_entries, FileEntry};
///
/// let entries = vec![
///     FileEntry {
///         name: "hello.txt".to_string(),
///         data: b"Hello World".to_vec(),
///         is_dir: false
///     }
/// ];
///
/// let archive = pack_entries(entries);
/// # assert!(!archive.is_empty());
/// ```
pub fn pack_entries(entries: Vec<FileEntry>) -> Vec<u8> {
    pack_from_file_entries(entries)
}

/// Unpacks the archive and creates directories/files on the specified path.
///
/// # Arguments
///
/// * `archive` - The binary archive data to unpack.
/// * `path` - The path to unpack the archive content to.
///
/// # Examples
///
/// ```no_run
/// use press_rs::packager::unpack;
/// use std::fs;
///
/// let archive_bytes = fs::read("backup.press").unwrap();
///
/// // Unpack archive content into the "output" directory
/// unpack(archive_bytes, "./output");
/// ```
pub fn unpack(archive: Vec<u8>, path: impl AsRef<Path>) {
    if !path.as_ref().exists() {
        fs::create_dir_all(path.as_ref()).expect("Cannot create directory");
    }
    unpack::unpack_with_dir_creation(archive, path);
}

/// Returns a list of unpacked entries. Does not create directories.
///
/// # Arguments
///
/// * `archive` - The binary archive data to unpack.
///
/// # Returns
///
/// Returns a `Vec<FileEntry>` containing the unpacked entries.
///
/// # Examples
///
/// ```no_run
/// use press_rs::packager::unpack_to_entries;
/// use std::fs;
///
/// let archive_bytes = fs::read("data.press").unwrap();
/// let entries = unpack_to_entries(archive_bytes);
///
/// for entry in entries {
///     println!("Found file: {}", entry.name);
/// }
/// ```
pub fn unpack_to_entries(archive: Vec<u8>) -> Vec<FileEntry> {
    unpack_to_file_entries(archive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_pack_unpack_in_memory_integration_flow() {
        let original_entries = vec![FileEntry {
            name: "a/b/c.txt".to_string(),
            data: b"nested content".to_vec(),
            is_dir: false,
        }];

        let archive = crate::packager::pack_entries(original_entries.clone());
        let unpacked_entries = unpack_to_entries(archive);

        assert_eq!(original_entries.len(), unpacked_entries.len());
        assert_eq!(original_entries[0].name, unpacked_entries[0].name);
        assert_eq!(original_entries[0].data, unpacked_entries[0].data);
    }

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
