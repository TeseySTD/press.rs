pub mod compressor;
pub mod packager;

#[cfg(test)]
mod tests {
    use crate::compressor::{compress_raw, decompress_raw};

    mod compression_quality_tests {
        use super::*;

        #[test]
        fn test_compression_ratio_text() {
            // Arrange
            let text = "hello world ".repeat(100);
            let input = text.as_bytes();
            // Act
            let compressed = compress_raw(input);
            let decompressed = decompress_raw(&compressed);
            let ratio = input.len() as f64 / compressed.len() as f64;

            // Assert
            assert!(ratio > 2.0, "Text compression ratio too low: {:.2}x", ratio);
            assert_eq!(input, &decompressed[..]);
        }

        #[test]
        fn test_compression_ratio_binary_pattern() {
            // Arrange
            let mut input = Vec::new();
            for i in 0..1000 {
                input.push((i % 10) as u8);
            }

            // Act
            let compressed = compress_raw(&input);
            let decompressed = decompress_raw(&compressed);
            let ratio = input.len() as f64 / compressed.len() as f64;

            // Assert
            assert!(
                ratio > 1.5,
                "Binary pattern compression too low: {:.2}x",
                ratio
            );
            assert_eq!(input, decompressed);
        }

        #[test]
        fn test_compression_ratio_json_like() {
            // Arrange
            let json = r#"{"name": "test", "value": 123, "active": true}"#.repeat(50);
            let input = json.as_bytes();

            // Act
            let compressed = compress_raw(input);
            let decompressed = decompress_raw(&compressed);
            let ratio = input.len() as f64 / compressed.len() as f64;

            // Assert
            assert!(
                ratio > 3.0,
                "JSON compression too low: {:.2}x (expected >3x)",
                ratio
            );
            assert_eq!(input, &decompressed[..]);
        }

        #[test]
        fn test_compression_ratio_random_data() {
            use rand::Rng;
            // Arrange
            let mut rng = rand::rng();
            let input: Vec<u8> = (0..10000).map(|_| rng.random()).collect();

            // Act
            let compressed = compress_raw(&input);
            let decompressed = decompress_raw(&compressed);
            let ratio = input.len() as f64 / compressed.len() as f64;

            // Assert
            assert!(
                ratio > 0.5,
                "Random data expanded too much: {:.2}x (some overhead expected)",
                ratio
            );
            assert_eq!(input, decompressed);
        }

        #[test]
        fn test_compression_ratio_already_compressed() {
            use rand::Rng;
            // Arrange
            let mut rng = rand::rng();
            let data: Vec<u8> = (0..5000).map(|_| rng.random()).collect();

            // Act
            let compressed = compress_raw(&data);
            let compressed_again = compress_raw(&compressed);
            let ratio = compressed.len() as f64 / compressed_again.len() as f64;

            // Assert
            assert!(
                ratio > 0.6,
                "Pre-compressed data expanded too much: {:.2}x",
                ratio
            );
        }

        #[test]
        fn test_compression_ratio_zeroes() {
            // Arrange
            let input = vec![0u8; 10000];

            // Act
            let compressed = compress_raw(&input);
            let decompressed = decompress_raw(&compressed);
            let ratio = input.len() as f64 / compressed.len() as f64;

            // Assert
            assert!(
                ratio > 50.0,
                "All zeros should compress VERY well: {:.2}x",
                ratio
            );
            assert_eq!(input, decompressed);
        }


        #[test]
        fn test_compression_ratio_mixed_content() {
            // Arrange
            let mut input = Vec::new();

            input.extend_from_slice(b"Hello World! ".repeat(50).as_slice());

            // Binary pattern
            for i in 0..500 {
                input.push((i % 256) as u8);
            }

            // Some random noise
            use rand::Rng;
            let mut rng = rand::rng();
            for _ in 0..100 {
                input.push(rng.random());
            }

            // Act
            let compressed = compress_raw(&input);
            let decompressed = decompress_raw(&compressed);
            let ratio = input.len() as f64 / compressed.len() as f64;

            // Assert
            assert!(ratio > 1.2, "Mixed content should compress: {:.2}x", ratio);
            assert_eq!(input, decompressed);
        }
    }

    mod compression_correctness_tests {
        use super::*;

        #[test]
        fn test_compression_empty() {
            // Arrange
            let input = b"";

            // Act
            let compressed = compress_raw(input);
            let decompressed = decompress_raw(&compressed);
            
            // Assert
            assert_eq!(input, &decompressed[..]);
        }

        #[test]
        fn test_roundtrip_various_sizes() {
            for size in [0, 1, 10, 100, 1000, 10000, 100000] {
                // Arrange
                let input = vec![0xAB; size];
                
                // Act
                let compressed = compress_raw(&input);
                let decompressed = decompress_raw(&compressed);

                // Assert
                assert_eq!(input, decompressed, "Roundtrip failed for size {}", size);
            }
        }

        #[test]
        fn test_all_byte_values() {
            // Arrange
            let input: Vec<u8> = (0..=255).cycle().take(1000).collect();

            // Act
            let compressed = compress_raw(&input);
            let decompressed = decompress_raw(&compressed);

            // Assert
            assert_eq!(input, decompressed);
        }
    }
}
