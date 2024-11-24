pub mod morphological;

pub trait Spell {
    /// Spell은 문장 단위로 작업을 처리
    fn cast(&self, input: &str) -> String;
}