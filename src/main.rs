use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process;

use bsg::BsgConfig;

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

fn process(contents: &str) -> String {
    for line in contents.lines() {
        let line_string = String::new();
        let tokens_iter = line.split_whitespace();
        for token in tokens_iter {
           line_string.lines(token); 
        }
    }

    String::new()
}

fn run(cfg: BsgConfig) -> Result<(), &'static str> {
    let contents = read(&cfg.src_file).map_err(|_err| "failed to read file")?;

    let contents = process(&contents);

    write(&cfg.dst_file, &contents).map_err(|_err| "failed to write file")?;

    // whats next?
    // create template html file
    // replace # with <h1></h1>
    // ## with 2
    // regular paragraphs with p

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = BsgConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
