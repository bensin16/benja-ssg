pub struct SsgConfig {
    pub src_file: String,
    pub dst_file: String,
}

impl SsgConfig {
    pub fn build(args: &[String]) -> Result<SsgConfig, &'static str> {
        if args.len() < 2 {
            return Err("failue");
        }

        let src_file = args[1].clone();
        let dst_file = src_file.clone();
        let dst_file = dst_file.replace(".md", ".html");

        println!("{src_file}");
        println!("{dst_file}");

        Ok(SsgConfig { src_file, dst_file })
    }
}
