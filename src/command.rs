use getopts::Options;

#[cfg(unix)]
const DEFAULT_DICT_PATH: &'static str = "/usr/share/dict/words";

#[cfg(windows)]
const DEFAULT_DICT_PATH: &'static str  = "C:\\Users\\ja\\Documents\\enable1.txt";

pub struct Command {
    targ_path: String,
    dict_path: String,
}

impl Command {
    pub fn from_args() -> Command {
        let mut opts = Options::new();
        opts.optopt("w", "words", "set dictionary file path", "WORDS");
        opts.optflag("p", "print", "print dictionary contents");

        let args: Vec<_> = ::std::env::args().collect();
        match opts.parse(&args[1..]) {
            Ok(ref matches) if matches.free.len() == 1 => Command {
                targ_path: matches.free[0].clone(),
                dict_path: matches.opt_str("w").unwrap_or(DEFAULT_DICT_PATH.to_owned()),
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
