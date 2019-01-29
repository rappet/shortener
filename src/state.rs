use std::collections::HashMap;

pub struct State {
    entries: HashMap<String, Entry>,
}

impl State {
    pub fn new() -> State {
        State {
            entries: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, key: &str, url: Entry) {
        self.entries.insert(key.to_owned(), url);
    }

    pub fn find_mapping(&self, key: &str) -> Option<&Entry> {
        self.entries.get(key)
    }
}

pub struct Entry {
    pub destination: String,
}
