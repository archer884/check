#[cfg_attr(test, feature(test))]
#[cfg(test)] extern crate test;
#[cfg(test)] mod bench;

extern crate getopts;
extern crate rayon;

use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::par_iter::{IntoParallelIterator, ParallelIterator};

mod command;
mod dictionary;
mod input;

use command::Command;
use dictionary::Dictionary;
use input::Line;

// Trait bounds are not (yet) enforced in type definitions:
// type Sources<S: Dictionary> = Vec<S>;
type Sources<S> = Vec<S>;

enum Error {
    Arguments = 1,
    Dictionary = 2,
    Input = 3,
}

pub fn main() {
    let command = Command::from_args();
    let sources = load_sources(&command);

    let input: Vec<_> = match File::open(command.targ_path()).map(|file| BufReader::new(file)) {
                            Ok(reader) => {
                                reader.lines()
                                      .filter_map(|l| l.ok())
                                      .enumerate()
                                      .filter(|&(_, ref s)| s.trim().len() > 0)
                                      .map(|source| Line::from_source(source))
                            }

                            _ => {
                                println!("Unable to load input: {}", command.targ_path());
                                std::process::exit(Error::Input as i32);
                            }
                        }
                        .collect();

    let mut errors = Vec::new();
    input.into_par_iter()
         .map(|line| line.errors(|word| is_error(word, &sources)))
         .collect_into(&mut errors);

    for error in errors.iter().flat_map(|errors| errors) {
        if command.with_lines() {
            println!("{}", error);
        } else {
            println!("{}", error.content());
        }
    }
}

fn is_error<S: Dictionary>(word: &str, sources: &Sources<S>) -> bool {
    sources.iter().all(|source| !source.is_valid(&word.to_lowercase()))
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
        Ok(reader) => {
            let mut dict: Vec<_> = reader.lines()
                                         .filter_map(|line| line.ok())
                                         .map(|line| line.trim().to_owned())
                                         .collect();

            dict.sort();
            dict
        }

        _ => {
            println!("Unable to load dictionary: {}", command.dict_path());
            std::process::exit(Error::Dictionary as i32);
        }
    }
}
