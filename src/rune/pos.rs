use std::collections::HashMap;

use super::Rune;

pub struct PosTaggingRune {
    dictionary: HashMap<String, String>,
}

impl PosTaggingRune {
    pub fn new(dictionary: HashMap<String, String>) -> Self {
        Self { dictionary }
    }
}

impl Rune for PosTaggingRune {
    fn cast(&self, input: &str) -> String {
        input
            .lines()
            .map(|word| {
                let tag = self
                    .dictionary
                    .get(word)
                    .cloned()
                    .unwrap_or("UNKNOWN".to_string());
                format!("{}:{}", word, tag)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
