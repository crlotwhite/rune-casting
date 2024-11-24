use super::Spell;

pub struct TokenizeSpell;

impl Spell for TokenizeSpell {
    fn cast(&self, input: &str) -> String {
        input.split_whitespace().collect::<Vec<&str>>().join("\n")
    }
}
