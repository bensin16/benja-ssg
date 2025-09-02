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

pub fn run(cfg: SsgConfig) -> Result<(), &'static str> {
    match cfg.src_file {
        Some(file_path) => {
            let contents = read(&file_path).map_err(|_err| "Failed to read input file")?;
            let contents = process_markdown(&contents);
            if let Some(dst) = &cfg.dst_file {
                write(&dst, &contents).map_err(|_err| "failed to write output file")?;
            } else {
                return Err("No destination file provided");
            }
            Ok(())
        }
        None => Err("No source file provided"),
    }
}
