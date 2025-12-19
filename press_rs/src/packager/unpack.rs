use std::{fs, path::Path};

use crate::packager::FileEntry;

use super::header::{ENTRY_SIZE, EntryType, Header};

pub fn unpack_to_file_entries(archive: Vec<u8>) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let mut i = 0;
    let mut block_was_empty = false;

    while i + ENTRY_SIZE <= archive.len() {
        let block = &archive[i..i + ENTRY_SIZE];

        if block_is_empty(block) {
            if block_was_empty {
                break;
            }
            block_was_empty = true;
            i += ENTRY_SIZE;
            continue;
        }
        block_was_empty = false;

        let mut bin_header = [0u8; ENTRY_SIZE];
        bin_header.copy_from_slice(block);
        let header = Header::from_bytes(bin_header);
        i += ENTRY_SIZE;

        let name = header.get_name();
        let entry_type = EntryType::new(header.typeflag[0]);

        match entry_type {
            EntryType::Directory => {
                entries.push(FileEntry {
                    name,
                    data: vec![],
                    is_dir: true,
                });
            }
            EntryType::File => {
                let size = header.get_size();
                let file_data = archive[i..i + size].to_vec();

                entries.push(FileEntry {
                    name,
                    data: file_data,
                    is_dir: false,
                });

                i += size;
                if size % ENTRY_SIZE != 0 {
                    i += ENTRY_SIZE - (size % ENTRY_SIZE);
                }
            }
        }
    }
    entries
}

/// Unpacks the archive and creates directories/files on the specified path.
pub fn unpack_with_dir_creation(archive: Vec<u8>, path: impl AsRef<Path>) {
    let mut i = 0;
    let mut block_was_empty = false;

    while i + ENTRY_SIZE <= archive.len() {
        if block_is_empty(&archive[i..i + ENTRY_SIZE]) && block_was_empty {
            println!("Finished unpacking");
            return;
        } else if block_is_empty(&archive[i..i + ENTRY_SIZE]) {
            block_was_empty = true;
            i += ENTRY_SIZE;
            continue;
        } else {
            block_was_empty = false;
        }

        let mut bin_header = [0u8; ENTRY_SIZE];
        bin_header.copy_from_slice(&archive[i..i + ENTRY_SIZE]);
        let header = Header::from_bytes(bin_header);
        i += ENTRY_SIZE;

        let name = header.get_name();
        let target_path = path.as_ref().join(name);

        match EntryType::new(header.typeflag[0]) {
            EntryType::Directory => {
                fs::create_dir_all(target_path).expect("Cannot create directory");
            }
            EntryType::File => {
                let size = header.get_size();

                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent).ok();
                }

                let file = archive[i..i + size].to_vec();
                fs::write(target_path, file).expect("Cannot write file");

                i += size;

                if size % ENTRY_SIZE != 0 {
                    i += ENTRY_SIZE - (size % ENTRY_SIZE);
                }
            }
        }
    }
}

fn block_is_empty(block: &[u8]) -> bool {
    block.iter().all(|&x| x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packager::header::{EntryType, Header};
    use std::fs;
    use tempfile::tempdir;

    mod memory_unpacking {
        use super::*;

        #[test]
        fn test_unpack_to_entries_standard_file() {
            // Arrange
            let mut header = Header::new();
            let file_name = "mem_test.txt";
            let content = b"memory content";
            header.set_name(file_name.to_string());
            header.set_size(content.len());
            header.set_typeflag(EntryType::File);

            let mut data = header.to_bytes().to_vec();
            data.extend_from_slice(content);

            // Act
            let entries = unpack_to_file_entries(data);

            // Assert
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].name, file_name);
            assert_eq!(entries[0].data, content);
            assert!(!entries[0].is_dir);
        }

        #[test]
        fn test_unpack_to_entries_empty_buffer() {
            // Arrange
            let data = vec![];
            // Act
            let entries = unpack_to_file_entries(data);
            // Assert
            assert_eq!(entries.len(), 0);
        }

        #[test]
        fn test_unpack_to_entries_corrupted_data() {
            // Arrange
            let corrupted = vec![0u8; ENTRY_SIZE - 1];
            // Act
            let entries = unpack_to_file_entries(corrupted);
            // Assert
            assert_eq!(entries.len(), 0);
        }
    }

    mod disk_unpacking {
        use super::*;

        #[test]
        fn test_unpack_file_entry() {
            // Arrange
            let mut header = Header::new();
            let file_name = "unpacked_unit.txt";
            let content = b"unit unpack content";

            header.set_name(file_name.to_string());
            header.set_size(content.len());
            header.set_typeflag(EntryType::File);

            let mut data = header.to_bytes().to_vec();
            data.extend_from_slice(content);

            let dir = tempdir().expect("Failed to create temp dir");

            // Act
            unpack_with_dir_creation(data, dir.path());

            // Assert
            let target_file = dir.path().join(file_name);
            assert!(target_file.exists());
            assert_eq!(fs::read(target_file).unwrap(), content);
        }

        #[test]
        fn test_unpack_empty_buffer() {
            // Arrange
            let empty_data = vec![];
            let dir = tempdir().expect("Failed to create temp dir");

            // Act
            unpack_with_dir_creation(empty_data, dir.path());

            // Assert
            let files_count = fs::read_dir(dir.path()).unwrap().count();
            assert_eq!(files_count, 0);
        }

        #[test]
        fn test_unpack_corrupted_short_data() {
            // Arrange
            let corrupted_data = vec![0u8; ENTRY_SIZE - 1]; // Data shorter than ENTRY_SIZE
            let dir = tempdir().expect("Failed to create temp dir");

            // Act & Assert
            // Should not panic due to the boundary check
            unpack_with_dir_creation(corrupted_data, dir.path());

            let files_count = fs::read_dir(dir.path()).unwrap().count();
            assert_eq!(
                files_count, 0,
                "No files should be created from corrupted data"
            );
        }
    }
}
