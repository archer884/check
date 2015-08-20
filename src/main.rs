
#[macro_use] extern crate lazy_static;

extern crate getopts;
extern crate num_cpus;
extern crate threadpool;

mod command;
mod input;

use command::Command;
use input::Line;

use std::collections::HashSet;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::sync::mpsc;
use threadpool::ThreadPool;

enum Error {
    Arguments = 1,
    Dictionary = 2,
    Input = 3,
}

lazy_static! {
    static ref COMMAND: Command = Command::from_args();
    static ref DICTIONARY: HashSet<String> = load_dictionary();
}

fn main() {
    let mut input = match File::open(COMMAND.targ_path()).map(|file| BufReader::new(file)) {
        Ok(reader) => reader.lines().filter_map(|l| l.ok()).enumerate().map(|source| Line::from_source(source)),
        _ => {
            println!("Unable to load input: {}", COMMAND.targ_path());
            std::process::exit(Error::Input as i32);
        }
    };

    process_input_parallel(&mut input);
}

fn process_input_parallel<I: Iterator<Item=Line>>(input: &mut I) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = mpsc::channel();

    let mut work_pieces = 0;
    for line in input {
        let tx = tx.clone();

        work_pieces += 1;
        pool.execute(move || {
            let mut errors = line.words();
            errors.retain(|word| !DICTIONARY.contains(&word.content));

            match errors.len() {
                0 => tx.send(None).unwrap(),
                _ => tx.send(Some(errors)).unwrap(),
            };
        })
    }

    let mut count = 0;
    for _ in 0..work_pieces {
        if let Some(errors) = rx.recv().unwrap() {
            for error in errors.into_iter() {
                count += 1;
                println!("{}", error);
            }
        }
    }

    if count == 0 {
        println!("No problems");
    }
}

fn load_dictionary() -> HashSet<String> {
    match File::open(COMMAND.dict_path()).map(|file| BufReader::new(file)) {
        Ok(reader) => reader.lines().filter_map(|line| line.ok()).map(|line| line.trim().to_owned()).collect(),
        _ => {
            println!("Unable to load dictionary: {}", COMMAND.dict_path());
            std::process::exit(Error::Dictionary as i32);
        }
    }
}
