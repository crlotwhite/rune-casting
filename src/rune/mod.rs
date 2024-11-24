pub mod uppercase;

pub use uppercase::UppercaseRune;

pub trait Rune {
    fn cast(&self, input: &str) -> String;
}
