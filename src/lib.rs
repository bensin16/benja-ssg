mod md_parser;
mod ssg_config;

// i dont think i want this public? but my doctests fail if i dont.
// maybe the md parser should be its own public api or something
pub use md_parser::process_markdown;
pub use ssg_config::SsgConfig;

use std::fs::File;
use std::io::{BufReader, Read, Write};

fn read(file_name: &str) -> std::io::Result<String> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write(file_name: &str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn run(cfg: SsgConfig) -> Result<(), String> {
    let contents =
        read(&cfg.src_file).map_err(|_err| format!("failed to read file {}", cfg.src_file))?;

    let contents = process_markdown(&contents);

    write(&cfg.dst_file, &contents).map_err(|_err| "failed to write file")?;

    Ok(())
}
