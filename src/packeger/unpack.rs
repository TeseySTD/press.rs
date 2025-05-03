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
                println!("size:{} bytes",size);
                let mut file = archive[i..i + size].to_vec();
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
