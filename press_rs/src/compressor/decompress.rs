use std::{
    cmp::Ordering,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::compressor::{INITIAL_CODE_WIDTH, MAX_CODE_WIDTH, MAX_ENTRY_COUNT};

struct BitReader<R>
where
    R: Read,
{
    read: R,
    buffer: u32,
    read_buffer: [u8; 1],
    cursor: u8,
}

impl<R> BitReader<R>
where
    R: Read,
{
    fn new(read: R) -> Self {
        Self {
            read,
            buffer: 0,
            read_buffer: [0; 1],
            cursor: 0,
        }
    }

    #[inline(always)]
    fn read_one(&mut self, width: u8) -> Result<u16, std::io::Error> {
        while self.cursor < width {
            match self.read.read_exact(&mut self.read_buffer[..]) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            self.buffer |= (self.read_buffer[0] as u32) << self.cursor;
            self.cursor += 8;
        }

        let mask = (1 << width) - 1;
        let data = (self.buffer & mask) as u16;
        self.buffer >>= width;
        self.cursor -= width;
        Ok(data)
    }
}

/// Core LZW decompression logic that works with any Read source
pub fn lzw_decompress_from_reader<R: Read>(reader: R) -> Vec<u8> {
    const MAX_TABLE_SIZE: usize = MAX_ENTRY_COUNT - 1;
    const MAX_STACK_SIZE: usize = MAX_TABLE_SIZE;

    let mut prefix: [u16; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];
    let mut suffix: [u8; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];
    let mut length: [usize; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];
    let mut decoding_stack: [u8; MAX_STACK_SIZE] = [0; MAX_STACK_SIZE];

    let mut reader = BitReader::new(reader);
    let mut output = Vec::new();

    for code in 0..1 << INITIAL_CODE_WIDTH {
        suffix[code as usize] = code as u8;
        length[code as usize] = 1;
    }

    let mut read_size = INITIAL_CODE_WIDTH + 1;
    let clear_code = 1 << INITIAL_CODE_WIDTH;
    let end_of_information = clear_code + 1;

    let mut size_increase_mask = 1 << read_size;
    let mut next_index = clear_code + 2;
    let mut previous_code: Option<u16> = None;
    let mut word_length = 0;

    loop {
        let code = match reader.read_one(read_size) {
            Ok(c) => c,
            Err(_) => break, // EOF reached
        };

        if code == clear_code {
            read_size = INITIAL_CODE_WIDTH + 1;
            size_increase_mask = 1 << read_size;
            next_index = clear_code + 2;
            previous_code = None;
            continue;
        } else if code == end_of_information {
            break;
        } else if previous_code == None {
            output.push(suffix[code as usize]);
            previous_code = Some(code);
            decoding_stack[0] = code as u8;
            word_length = 1;
            continue;
        }

        let initial_code = code;

        match code.cmp(&next_index) {
            Ordering::Greater => {
                panic!("Invalid code: {}", code);
            }
            Ordering::Equal => {
                // KwKwK fix
                decoding_stack[word_length] = decoding_stack[0];
                word_length += 1;
            }
            Ordering::Less => {
                word_length = length[code as usize];
                let mut stack_top = word_length;
                let mut temp_code = code;

                while temp_code >= clear_code {
                    stack_top -= 1;
                    if stack_top == 0 {
                        break;
                    }
                    decoding_stack[stack_top] = suffix[temp_code as usize];
                    temp_code = prefix[temp_code as usize];
                }
                decoding_stack[0] = temp_code as u8;
            }
        }

        output.extend_from_slice(&decoding_stack[0..word_length]);

        if (next_index as usize) < MAX_TABLE_SIZE {
            prefix[next_index as usize] = previous_code.unwrap();
            suffix[next_index as usize] = decoding_stack[0];
            length[next_index as usize] = length[previous_code.unwrap() as usize] + 1;
            next_index += 1;

            if next_index == size_increase_mask && read_size < MAX_CODE_WIDTH {
                read_size += 1;
                size_increase_mask = 1 << read_size;
            }
        }

        previous_code = Some(initial_code);
    }

    output
}

/// Decompress from a byte slice (in-memory)
pub fn lzw_decompress_bytes(data: &[u8]) -> Vec<u8> {
    use std::io::Cursor;
    lzw_decompress_from_reader(Cursor::new(data))
}

/// Convenience function for decompressing from a file path
pub fn lzw_decompress(path: impl AsRef<Path>) -> Vec<u8> {
    let file = File::open(path).expect("Cannot open file");
    lzw_decompress_from_reader(BufReader::new(file))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Cursor;

    mod bit_reader {
        use super::*;

        #[test]
        fn test_read_variable_widths() {
            // Arrange
            let data = vec![0xFF, 0x0F];
            let cursor = Cursor::new(data);
            let mut reader = BitReader::new(cursor);

            // Act
            let val_12bits = reader.read_one(12).unwrap();
            let val_4bits = reader.read_one(4).unwrap();

            // Assert
            assert_eq!(val_12bits, 0xFFF);
            assert_eq!(val_4bits, 0x0);
        }

        #[test]
        fn test_read_exhaustion() {
            // Arrange
            let data = vec![0xAA]; // 10101010
            let cursor = Cursor::new(data);
            let mut reader = BitReader::new(cursor);

            // Act
            let val1 = reader.read_one(4);
            let val2 = reader.read_one(4);
            let val3 = reader.read_one(4);

            // Assert
            assert!(val1.is_ok());
            assert!(val2.is_ok());
            assert!(val3.is_err()); // EOF
        }
    }

    mod lzw_logic {
        use super::*;
        use crate::compressor::compress::lzw_compress;
        use rand::{Rng, rng};
        use tempfile::tempdir;

        // Helper for round-trip tests (file-based)
        fn run_round_trip_file(name: &str, input: &[u8]) {
            let dir = tempdir().expect("Failed to create temp dir");
            let compressed = lzw_compress(input);
            let temp_file_path = dir.path().join(format!("{}.lzw", name));
            fs::write(&temp_file_path, &compressed).expect("Failed to write temp file");

            let decompressed = lzw_decompress(temp_file_path.to_str().unwrap());

            assert_eq!(
                input,
                decompressed.as_slice(),
                "Mismatch for {}: len {} vs {}",
                name,
                input.len(),
                decompressed.len()
            );
        }

        // Helper for in-memory round-trip tests
        fn run_round_trip_memory(name: &str, input: &[u8]) {
            let compressed = lzw_compress(input);
            let decompressed = lzw_decompress_bytes(&compressed);

            assert_eq!(
                input,
                decompressed.as_slice(),
                "Mismatch for {}: len {} vs {}",
                name,
                input.len(),
                decompressed.len()
            );
        }

        #[test]
        fn test_short_text() {
            let data = b"TOBEORNOTTOBE";
            run_round_trip_file("short", data);
            run_round_trip_memory("short_mem", data);
        }

        #[test]
        fn test_binary_data() {
            let mut data = Vec::with_capacity(256);
            for i in 0..=255u8 {
                data.push(i);
            }
            run_round_trip_file("binary", &data);
            run_round_trip_memory("binary_mem", &data);
        }

        #[test]
        fn test_repetitive_growth() {
            let mut data = Vec::new();
            for _ in 0..100 {
                data.extend_from_slice(b"ABCDE");
            }
            run_round_trip_file("repetitive", &data);
            run_round_trip_memory("repetitive_mem", &data);
        }

        #[test]
        fn test_dictionary_overflow_and_reset() {
            let mut data = vec![0u8; 100_000];
            rng().fill(&mut data[..]);

            run_round_trip_file("overflow_random", &data);
            run_round_trip_memory("overflow_random_mem", &data);
        }

        #[test]
        fn test_single_byte() {
            run_round_trip_file("single", b"Z");
            run_round_trip_memory("single_mem", b"Z");
        }

        #[test]
        fn test_decompress_from_cursor() {
            // Test with io::Cursor directly
            let data = b"TESTDATA";
            let compressed = lzw_compress(data);
            let cursor = Cursor::new(compressed);
            let decompressed = lzw_decompress_from_reader(cursor);

            assert_eq!(data.as_slice(), decompressed.as_slice());
        }
    }
}
