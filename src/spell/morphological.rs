use super::Spell;

pub struct MorphologicalSpell {
    runes: Vec<Box<dyn crate::rune::Rune>>,
}

impl MorphologicalSpell {
    pub fn new() -> Self {
        Self { runes: Vec::new() }
    }

    pub fn add_rune<T: crate::rune::Rune + 'static>(&mut self, rune: T) {
        self.runes.push(Box::new(rune));
    }
}

impl Spell for MorphologicalSpell {
    fn cast(&self, input: &str) -> String {
        self.runes
            .iter()
            .fold(input.to_string(), |acc, rune| rune.cast(&acc))
    }
}

/// 테스트
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PosTaggingRune, TokenizerRune};
    use std::collections::HashMap;

    #[test]
    fn test_spell() {
        let mut spell = MorphologicalSpell::new();

        // Tokenizer 추가
        let tokenizer = TokenizerRune;
        spell.add_rune(tokenizer);

        // 품사 태깅 추가
        let mut dictionary = HashMap::new();
        dictionary.insert("run".to_string(), "VB".to_string());
        dictionary.insert("fox".to_string(), "NN".to_string());
        spell.add_rune(PosTaggingRune::new(dictionary));

        let result = spell.cast("run fox");
        assert_eq!(result, "run:VB\nfox:NN");
    }
}