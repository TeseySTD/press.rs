use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_file_or_folder_size(path: impl AsRef<Path>) -> Result<u64, io::Error> {
    let path_ref = path.as_ref();
    if path_ref.is_file() {
        return match path_ref.metadata(){
            Ok(metadata) => Ok(metadata.len()),
            Err(e) => Err(e),
        };
    } else {
        return match dir_size(path_ref) {
            Ok(size) => Ok(size as u64),
            Err(e) => Err(e),
        };
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
