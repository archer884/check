#![feature(test)]
#![allow(unused_features)] // disables warning for unused test feature

#[macro_use] extern crate lazy_static;

extern crate getopts;
extern crate num_cpus;
extern crate regex;
extern crate threadpool;

mod command;
mod input;

#[cfg(test)] extern crate test;
#[cfg(test)] mod bench;

use command::Command;
use input::Line;

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
    static ref DICTIONARY: Vec<String> = load_dictionary();
}

pub fn main() {
    let mut input = match File::open(COMMAND.targ_path()).map(|file| BufReader::new(file)) {
        Ok(reader) => reader.lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .filter(|&(_, ref s)| s.trim().len() > 0)
            .map(|source| Line::from_source(source)),

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
            // The word pattern allows periods to be stored with words because they are
            // part of some "words," like "St." and "Mr." This second stage allows us to
            // determine if a word would have been found in the dictionary had we not stuck
            // a period on it for no damn reason.
            let mut errors = line.words();
            errors.retain(|word| DICTIONARY.binary_search(&word.content.to_lowercase()).is_err()
                && DICTIONARY.binary_search(&word.content.trim_right_matches('.').to_lowercase()).is_err());

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

fn load_dictionary() -> Vec<String> {
    match File::open(COMMAND.dict_path()).map(|file| BufReader::new(file)) {
        Ok(reader) => reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| line.trim().to_owned())
            .collect(),

        _ => {
            println!("Unable to load dictionary: {}", COMMAND.dict_path());
            std::process::exit(Error::Dictionary as i32);
        }
    }
}
