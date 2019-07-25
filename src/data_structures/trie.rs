//! Implement a Trie data structure

use std::collections::HashMap;

pub struct Trie {
    flag: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie { flag: false, children: HashMap::new() }
    }

    pub fn search(&self, value: &str) -> bool {
        if value.is_empty() {
            self.flag
        } else {
            match self.children.get(&value.chars().next().unwrap()) {
                Some(child) => child.search(&value[1..]),
                None => false,
            }
        }
    }

    pub fn insert(&mut self, value: &str) {
        if value.is_empty() {
            self.flag = true;
        } else {
            match self.children.get_mut(&value.chars().next().unwrap()) {
                Some(child) => {
                    child.insert(&value[1..]);
                    return;
                },
                None => {},
            };
            let mut child = Trie::new();
            child.insert(&value[1..]);
            self.children.insert(value.chars().next().unwrap(), child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    fn make_trie() -> Trie {
        let mut trie = Trie::new();
        trie.insert("rust");
        trie.insert("python");
        trie.insert("pascal");
        trie.insert("c");
        trie.insert("ruby");
        trie.insert("c++");
        trie.insert("java");
        trie.insert("javascript");
        trie.insert("c#");
        trie.insert("nawk");
        trie.insert("fortran");
        trie.insert("perl");
        trie
    }

    #[test]
    fn test_search() {
        let trie = make_trie();
        assert!(trie.search("c++"));
        assert!(trie.search("c"));
        assert!(trie.search("rust"));
        assert!(trie.search("nawk"));
        assert!(trie.search("perl"));
        assert!(!trie.search("html"));
        assert!(!trie.search("for"));
        assert!(!trie.search("regex"));
    }
}
