use std::collections::HashMap;

use super::Spell;

pub struct PosTaggingSpell {
    dictionary: HashMap<String, String>,
}

impl PosTaggingSpell {
    pub fn new(dictionary: HashMap<String, String>) -> Self {
        Self { dictionary }
    }
}

impl Spell for PosTaggingSpell {
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
