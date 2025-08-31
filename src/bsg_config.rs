pub struct BsgConfig {
    pub src_file: String,
    pub dst_file: String,
}

impl BsgConfig {
    pub fn build(args: &[String]) -> Result<BsgConfig, &'static str> {
        if args.len() < 3 {
            return Err("failue");
        }

        let src_file = args[1].clone();
        let dst_file = args[2].clone();

        Ok(BsgConfig { src_file, dst_file })
    }
}
