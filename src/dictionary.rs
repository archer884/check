use std::collections::HashSet;

pub trait Dictionary: Sync {
    fn is_valid(&self, word: &String) -> bool;
}

impl Dictionary for Vec<String> {
    fn is_valid(&self, word: &String) -> bool {
        // self.binary_search(word).is_ok()
        match self.binary_search(word) {
            Ok(_) => true,
            Err(idx) => {
                println!("{} :: {}", word, self[idx]);
                false
            }
        }
    }
}

impl Dictionary for HashSet<String> {
    fn is_valid(&self, word: &String) -> bool {
        self.contains(word)
    }
}
