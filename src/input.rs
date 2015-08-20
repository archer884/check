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
            .map(|word| Word {
                number: self.number,
                content: word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase().to_owned(),
            }).collect()
    }
}

pub struct Word {
    pub number: usize,
    pub content: String,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {:4}: {}", self.number, self.content)
    }
}
