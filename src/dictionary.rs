use std::collections::HashSet;

pub trait Dictionary: Sync {
    fn init(&mut self);
    fn is_valid(&self, word: &String) -> bool;
}

impl Dictionary for Vec<String> {
    fn init (&mut self) {
        self.sort()
    }
    
    fn is_valid(&self, word: &String) -> bool {
        self.binary_search(word).is_ok()
    }
}

impl Dictionary for HashSet<String> {
    fn init(&mut self) { }

    fn is_valid(&self, word: &String) -> bool {
        self.contains(word)
    }
}
