use getopts::Options;

pub struct Command {
    targ_path: String,
    dict_path: String,
}

impl Command {
    pub fn from_args() -> Command {
        let mut opts = Options::new();
        opts.optopt("w", "words", "set dictionary file path", "WORDS");

        let args: Vec<_> = ::std::env::args().collect();
        match opts.parse(&args[1..]) {
            Ok(ref matches) if matches.free.len() == 1 => Command {
                targ_path: matches.free[0].clone(),
                dict_path: matches.opt_str("w").unwrap_or("C:\\Users\\ja\\Documents\\enable1.txt".to_owned()),
            },
            Ok(_) | Err(_) => {
                println!("Invalid arguments: {:?}", args);
                ::std::process::exit(super::Error::Arguments as i32);
            }
        }
    }

    pub fn dict_path(&self) -> &str {
        &self.dict_path
    }

    pub fn targ_path(&self) -> &str {
        &self.targ_path
    }
}
