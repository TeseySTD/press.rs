use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use press_rs::compressor::{compress_raw, decompress_raw};
use press_rs::packager::{pack_entries, unpack_to_entries, FileEntry};

use rand::Rng;

fn generate_data(size: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random()).collect()
}

fn generate_entries(count: usize, file_size: usize) -> Vec<FileEntry> {
    (0..count)
        .map(|i| FileEntry {
            name: format!("file_{}.bin", i),
            data: generate_data(file_size),
            is_dir: false,
        })
        .collect()
}

fn bench_compress_raw(c: &mut Criterion) {
    let mut group = c.benchmark_group("compress_raw");

    for size in [1024, 100 * 1024, 1024 * 1024].iter() {
        let data = generate_data(*size);

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &data,
            |b, data| b.iter(|| compress_raw(black_box(data))),
        );
    }
    group.finish();
}

fn bench_decompress_raw(c: &mut Criterion) {
    let mut group = c.benchmark_group("decompress_raw");

    for size in [1024, 100 * 1024].iter() {
        let original_data = generate_data(*size);
        let compressed_data = compress_raw(&original_data);

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &compressed_data,
            |b, data| b.iter(|| decompress_raw(black_box(data))),
        );
    }
    group.finish();
}

fn bench_pack_entries(c: &mut Criterion) {
    let mut group = c.benchmark_group("pack_entries");

    let entries = vec![
        FileEntry {
            name: "test1.txt".to_string(),
            data: generate_data(1024),
            is_dir: false,
        },
        FileEntry {
            name: "photos/image.bin".to_string(),
            data: generate_data(10 * 1024),
            is_dir: false,
        },
        FileEntry {
            name: "folder/".to_string(),
            data: vec![],
            is_dir: true,
        },
    ];

    group.bench_function("pack_small_entries", |b| {
        b.iter_with_setup(
            || entries.clone(),
            |entries| pack_entries(black_box(entries)),
        )
    });

    group.finish();
}

fn bench_unpack_entries(c: &mut Criterion) {
    let mut group = c.benchmark_group("unpack_entries");

    let entries = vec![FileEntry {
        name: "test.txt".to_string(),
        data: generate_data(5 * 1024),
        is_dir: false,
    }];
    let archive = pack_entries(entries);

    group.bench_function("unpack_small_archive", |b| {
        b.iter(|| unpack_to_entries(black_box(archive.clone())))
    });

    group.finish();
}

fn bench_full_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_compression");

    let total_size = 10 * 10 * 1024;
    let entries = generate_entries(10, 10 * 1024);

    group.throughput(Throughput::Bytes(total_size as u64));

    group.bench_function("pack_and_compress_100kb", |b| {
        b.iter_with_setup(
            || entries.clone(),
            |entries| {
                let packed = pack_entries(entries);
                compress_raw(black_box(&packed))
            },
        )
    });

    group.finish();
}

fn bench_full_decompression(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_decompression");

    let entries = generate_entries(10, 10 * 1024);
    let packed = pack_entries(entries);
    let compressed = compress_raw(&packed);
    let total_size = packed.len() as u64;

    group.throughput(Throughput::Bytes(total_size));

    group.bench_function("decompress_and_unpack_100kb", |b| {
        b.iter(|| {
            let decompressed = decompress_raw(black_box(&compressed));
            unpack_to_entries(black_box(decompressed))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_compress_raw,
    bench_decompress_raw,
    bench_pack_entries,
    bench_unpack_entries,
    bench_full_compression,
    bench_full_decompression
);
criterion_main!(benches);
