use std::env;
use std::process;

use benja_ssg::SsgConfig;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = SsgConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = benja_ssg::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
