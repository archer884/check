use std::fmt;

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
        self.content.split_whitespace()
            .flat_map(|word| word.split('-'))
            .map(|word| Word {
                number: self.number,
                content: word.trim_matches(|c: char| !c.is_alphabetic()).to_owned(),
            })
            .filter(|word| word.content.len() > 0)
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
        write!(f, "Line {:4}: {}", self.number, self.content)
    }
}
