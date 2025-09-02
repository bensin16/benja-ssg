pub struct SsgConfig {
    pub help: bool,
    pub src_file: Option<String>,
    pub dst_file: Option<String>,
}

impl Default for SsgConfig {
    fn default() -> SsgConfig {
        SsgConfig {
            help: true,
            src_file: None,
            dst_file: None,
        }
    }
}

impl SsgConfig {
    pub fn build(args: &[String]) -> Result<SsgConfig, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments to build config");
        }

        for arg in args.iter() {
            if arg == "-h" || arg == "--help" {
                return Ok(SsgConfig {
                    help: true,
                    ..Default::default()
                });
            }
        }

        let src_file = args.iter().enumerate().find_map(|(i, arg)| {
            if arg == "-f" || arg == "--file" {
                args.get(i + 1).cloned()
            } else {
                None
            }
        });

        match src_file {
            Some(src_file) => {
                let dst_file = src_file.replace(".md", ".html");
                return Ok(SsgConfig {
                    help: false,
                    src_file: Some(src_file),
                    dst_file: Some(dst_file),
                });
            }
            None => return Err("No source file name provided"),
        }
    }
}
