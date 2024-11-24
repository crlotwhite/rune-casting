use super::Rune;

pub struct UppercaseRune;

impl Rune for UppercaseRune {
    fn cast(&self, input: &str) -> String {
        input.to_uppercase()
    }
}
