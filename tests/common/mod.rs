use std::fs::File;
use std::io::{BufReader, Read};

pub fn setup() {
    // setup
}

pub fn _cleanup() {
    // cleanup
}

pub fn read(file_name: &str) -> std::io::Result<String> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
