use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)      // create the file if it does not exist
        .append(true)      // append to the file
        .open(path)
        .expect("Failed to open or create the file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to the file");
}
