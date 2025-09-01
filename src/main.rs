use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process;

use benja_ssg::SsgConfig;

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

fn run(cfg: SsgConfig) -> Result<(), &'static str> {
    let contents = read(&cfg.src_file).map_err(|_err| "failed to read file")?;

    let contents = benja_ssg::process_markdown(&contents);

    write(&cfg.dst_file, &contents).map_err(|_err| "failed to write file")?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = SsgConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
