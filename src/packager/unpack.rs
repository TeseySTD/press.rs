use std::{fs, path::Path};

use super::header::{ENTRY_SIZE, EntryType, Header};

pub fn unpack(archive: Vec<u8>, path: impl AsRef<Path>) {
    let mut i = 0;
    let mut block_was_empty = false;

    while i < archive.len() {
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
    use std::path::Path;

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

        let output_path = Path::new("unit_test_unpack_out");

        // Act
        unpack(data, output_path);

        // Assert
        let target_file = output_path.join(file_name);
        assert!(target_file.exists());
        assert_eq!(fs::read(target_file).unwrap(), content);

        fs::remove_dir_all(output_path).ok();
    }

    #[test]
    fn test_unpack_empty_buffer() {
        // Arrange
        let empty_data = vec![];
        let output_path = Path::new("unit_test_unpack_empty");

        // Act
        // Unpacking empty data should not panic and should not create anything
        unpack(empty_data, output_path);

        // Assert
        assert!(!output_path.exists() || fs::read_dir(output_path).unwrap().count() == 0);

        fs::remove_dir_all(output_path).ok();
    }

    #[test]
    fn test_unpack_corrupted_short_data() {
        // Arrange
        // Data shorter than a single header
        let corrupted_data = vec![0u8; 10];
        let output_path = Path::new("unit_test_unpack_corrupt");

        // Act
        unpack(corrupted_data, output_path);

        // Assert
        assert!(!output_path.exists() || fs::read_dir(output_path).unwrap().count() == 0);

        fs::remove_dir_all(output_path).ok();
    }
}
