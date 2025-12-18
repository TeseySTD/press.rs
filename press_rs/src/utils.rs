use std::{
    fs, io,
    path::{Path, PathBuf},
};

const KI_B: f64 = 1024.0;
const MI_B: f64 = KI_B * KI_B;
const GI_B: f64 = KI_B * KI_B * KI_B;

pub fn print_with_size_formats(msg: &str, size: usize) {
    let size_f = size as f64;
    let mb = size_f / MI_B;
    let gb = size_f / GI_B;
    println!("{msg} (in 2 pow): {size} bytes, {mb:.3} MiB, {gb:.3} GiB");
}

pub fn get_file_or_folder_size(path: impl AsRef<Path>) -> Result<u64, io::Error> {
    let path_ref = path.as_ref();
    if path_ref.is_file() {
        return match path_ref.metadata() {
            Ok(metadata) => Ok(metadata.len()),
            Err(e) => Err(e),
        };
    } else {
        return dir_size(path_ref);
    };
}

pub fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    dir_size(fs::read_dir(path.into())?)
}
