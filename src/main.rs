#![feature(test)]
#![allow(unused_features)] // disables warning for unused test feature

extern crate getopts;
extern crate num_cpus;
extern crate scoped_threadpool;

mod command;
mod input;

#[cfg(test)] extern crate test;
#[cfg(test)] mod bench;

use command::Command;
use input::Line;

use scoped_threadpool::Pool;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::sync::mpsc;

type Sources = Vec<Vec<String>>;

enum Error {
    Arguments = 1,
    Dictionary = 2,
    Input = 3,
}

pub fn main() {
    let command = Command::from_args();
    let sources = load_sources(&command);

    let mut input = match File::open(command.targ_path()).map(|file| BufReader::new(file)) {
        Ok(reader) => reader.lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .filter(|&(_, ref s)| s.trim().len() > 0)
            .map(|source| Line::from_source(source)),

        _ => {
            println!("Unable to load input: {}", command.targ_path());
            std::process::exit(Error::Input as i32);
        }
    };

    process_input_parallel(&command, &mut input, &sources);
}

fn process_input_parallel<I>(command: &Command, input: &mut I, sources: &Sources) where
    I: Iterator<Item=Line>
{
    let mut pool = Pool::new(num_cpus::get() as u32);
    let (tx, rx) = mpsc::channel();

    let mut work_pieces = 0;
    for line in input {
        let tx = tx.clone();

        work_pieces += 1;
        pool.scoped(|scoped| scoped.execute(move || {
            let errors = line.errors(|word| is_error(word, sources));
            match errors.len() {
                0 => tx.send(None).unwrap(),
                _ => tx.send(Some(errors)).unwrap(),
            };
        }));
    }

    let errors = rx.iter()
        .take(work_pieces)
        .filter_map(|errors| errors)
        .flat_map(|errors| errors.into_iter());

    let mut count = 0;
    for error in errors {
        count += 1;
        match command.with_lines() {
            true => println!("{}", error),
            false => println!("{}", error.content()),
        }
    }

    if count == 0 {
        println!("No problems");
    }
}

fn is_error(word: &str, sources: &Sources) -> bool {
    sources.iter().all(|source| source.binary_search(&word.to_lowercase()).is_err())
}

fn load_sources(command: &Command) -> Vec<Vec<String>> {
    let mut sources = vec![load_sys_dict(command)];
    if let Ok(reader) = File::open("./.spelling").map(|file| BufReader::new(file)) {
        sources.push(reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| line.trim().to_owned())
            .collect());
    }
    sources
}

fn load_sys_dict(command: &Command) -> Vec<String> {
    match File::open(command.dict_path()).map(|file| BufReader::new(file)) {
        Ok(reader) => reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| line.trim().to_owned())
            .collect(),

        _ => {
            println!("Unable to load dictionary: {}", command.dict_path());
            std::process::exit(Error::Dictionary as i32);
        }
    }
}
