mod utils;

use press_rs::compressor::{EXTENSION, compress, decompress};
use std::{
    io::{self, Write},
    path::Path,
};
use utils::{get_file_or_folder_size, print_with_size_formats};

fn main() {
    loop {
        println!("\n--- PressRS Menu ---");
        match prompt("1. Compress\n2. Decompress\n(q to quit)\n>> ").as_str() {
            "1" => run_compress(),
            "2" => run_decompress(),
            "q" | "exit" => break,
            _ => println!("Invalid option"),
        }
    }
}

fn run_compress() {
    let input = prompt("Path to compress: ");
    let path = Path::new(&input);

    if !path.exists() {
        return println!("Error: Path does not exist.");
    }

    let dest = path.with_extension(EXTENSION);
    println!("Compressing to: {:?}", dest);

    match compress(path) {
        Ok(compressed_data) => {
            if let Err(e) = std::fs::write(&dest, &compressed_data) {
                println!("Failed to write file: {}", e);
                return;
            }

            let original_size = get_file_or_folder_size(path).unwrap_or(0) as usize;
            print_with_size_formats("Original size", original_size);
            print_with_size_formats("Compressed size", compressed_data.len());
        }
        Err(e) => println!("Compression failed: {}", e),
    }
}

fn run_decompress() {
    let input = prompt("Path to decompress: ");
    let path = Path::new(&input);

    if !path.exists() {
        return println!("Error: Path does not exist.");
    }

    let output_dir = "./test_data/decompressed";

    std::fs::create_dir_all(output_dir).ok();

    println!("Decompressing...");
    decompress(path, output_dir);
    println!("Done. Output in: {}", output_dir);
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("Flush failed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read failed");
    input.trim().to_string()
}
