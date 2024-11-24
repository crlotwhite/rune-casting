pub mod pos;
pub mod tokenizer;

pub trait Rune {
    fn cast(&self, input: &str) -> String;
}
