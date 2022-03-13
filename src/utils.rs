use std::env;
use std::io;
use std::path::PathBuf;

pub fn get_current_dir() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

pub fn get_full_file_path(identifier: &str) -> std::path::PathBuf {
    let mut path = PathBuf::new();
    path.push(get_current_dir().unwrap());
    path.push(identifier);
    return path;
}
