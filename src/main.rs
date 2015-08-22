#![feature(test)]
#![allow(unused_features)] // disables warning for unused test feature

extern crate getopts;
extern crate num_cpus;
extern crate threadpool;

mod command;
mod input;

#[cfg(test)] extern crate test;
#[cfg(test)] mod bench;

use command::Command;
use input::Line;

use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::sync::Arc;
use std::sync::mpsc;
use threadpool::ThreadPool;

enum Error {
    Arguments = 1,
    Dictionary = 2,
    Input = 3,
}

pub fn main() {
    let command = Command::from_args();
    let sources = Arc::new(load_sources(&command));

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

    process_input_parallel(&mut input, &sources);
}

fn process_input_parallel<I: Iterator<Item=Line>>(input: &mut I, sources: &Arc<Vec<Vec<String>>>) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = mpsc::channel();

    let mut work_pieces = 0;
    for line in input {
        let tx = tx.clone();
        let sources = sources.clone();

        work_pieces += 1;
        pool.execute(move || {
            let errors = line.errors(|word| is_error(word, &sources));
            match errors.len() {
                0 => tx.send(None).unwrap(),
                _ => tx.send(Some(errors)).unwrap(),
            };
        });
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

fn is_error(word: &str, sources: &Arc<Vec<Vec<String>>>) -> bool {
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
