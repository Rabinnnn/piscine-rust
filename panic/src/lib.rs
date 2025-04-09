use std::fs::File;
use std::path::Path;

pub fn open_file(s: &str) -> File {
    let path = Path::new(s);
    File::open(path).expect("File not found!")
}
