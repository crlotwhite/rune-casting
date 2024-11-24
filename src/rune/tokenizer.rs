use super::Rune;

pub struct TokenizerRune;

impl Rune for TokenizerRune {
    fn cast(&self, input: &str) -> String {
        input.split_whitespace().collect::<Vec<&str>>().join("\n")
    }
}
