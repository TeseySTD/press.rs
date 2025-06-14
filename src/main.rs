use std::{io::Write, path::Path};

use compressor::{EXTENSION, compress};

mod compressor;
mod packeger;
mod utils;

fn main() {
    loop {
        let choice = choice();

        if choice == "1" {
            print_immediatly("Enter path to compress: ");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Could not read line");
            let mut path = Path::new(input.trim());

            println!(
                "Location of compressed file will be: {}",
                path.with_extension(EXTENSION)
                    .to_str()
                    .expect("Cannot create path")
            );

            if !path.exists() {
                println!("Path does not exist");
                continue;
            } else {
                let compressed =
                    compress(&mut path).expect("Cannot compress file/folder using given path");

                let size = utils::get_file_or_folder_size(path).expect("Cannot get size for given path") as u64;
        
                let compressed_size = compressed.len();

                std::fs::write(path.with_extension(EXTENSION), &compressed)
                    .expect("Cannot write compressed file");

                println!("Original file size: {} bytes", size);
                println!("Compressed file size: {} bytes", compressed_size);
            }
        } else if choice == "2" {
            print_immediatly("Enter path to decompress: ");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Could not read line");

            let path = Path::new(input.trim());

            if !path.exists() {
                println!("Path does not exist");
                continue;
            } else {
                compressor::decompress(path, "./test/decompressed");
            }
        }
    }
}

fn print_immediatly(s: &str) {
    print!("{}", s);
    std::io::stdout().flush().expect("Could not flush stdout");
}

fn choice() -> String {
    let mut input = String::new();

    loop {
        println!("Choose an option:");
        println!("1. Compress");
        println!("2. Decompress");
        print_immediatly("Enter your choice: ");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        if input.trim() != "1" && input.trim() != "2" {
            println!("Invalid choice");
            continue;
        }

        return input.trim().to_string();
    }
}
