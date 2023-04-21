use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::clipboard::{ClipboardProvider, ClipboardType};
use helix_core::register::Register;

#[derive(Debug)]
struct ClipboardRegister {
    values: Vec<String>,
    last_content_hash: u64,
    clipboard_provider: Box<dyn ClipboardProvider>,
    clipboard_type: ClipboardType,
}

impl ClipboardRegister {
    pub fn new(
        clipboard_provider: Box<dyn ClipboardProvider>,
        clipboard_type: ClipboardType,
    ) -> Self {
        let values: Vec<String> = vec![];
        let joined = values.join("");
        Self {
            values,
            last_content_hash: Self::compute_hash(&joined),
            clipboard_provider,
            clipboard_type,
        }
    }

    fn compute_hash(contents: &str) -> u64 {
        let mut s = DefaultHasher::new();
        contents.hash(&mut s);
        s.finish()
    }

    fn has_same_content_hash(&self, contents: &str) -> bool {
        self.last_content_hash == Self::compute_hash(contents)
    }
}

impl ClipboardRegister {
    fn read(&self) -> Result<Vec<String>, ()> {
        let content = self.clipboard_provider.get_contents(self.clipboard_type);
        content.map(|contents| {
            if self.has_same_content_hash(&contents) {
                self.values.to_owned()
            } else {
                vec![contents]
            }
        })
    }

    fn write(&mut self, values: Vec<String>) {
        let contents = values.join("");
        self.last_content_hash = Self::compute_hash(&contents);
        let _ = self
            .clipboard_provider
            .set_contents(contents, self.clipboard_type);
        self.values = values;
    }
}
