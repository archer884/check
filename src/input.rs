use regex::Regex;
use std::fmt;

lazy_static! {
    static ref WORD_PATTERN: Regex = Regex::new(r"([A-z]+)('[A-z]+)?\.?").unwrap();
}

pub struct Line {
    pub number: usize,
    pub content: String,
}

impl Line {
    pub fn from_source(source: (usize, String)) -> Line {
        Line {
            number: source.0 + 1,
            content: source.1
        }
    }

    pub fn words(&self) -> Vec<Word> {
        WORD_PATTERN.captures_iter(&self.content)
            .filter_map(|cap| cap.at(0).map(|content| Word {
                number: self.number,
                content: content.to_owned()
            }))
            .collect()
    }
}

#[derive(Debug)]
pub struct Word {
    pub number: usize,
    pub content: String,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {:4}: {}", self.number, self.content.trim_right_matches('.'))
    }
}
