use rstest::rstest;
use rune_casting::spell::{Spell, TokenizeSpell, PosTaggingSpell};
use std::collections::HashMap;

#[rstest]
#[case("hello world", "hello\nworld")]
#[case("rust is awesome", "rust\nis\nawesome")]
#[case("single", "single")]
fn test_tokenize_spell(#[case] input: &str, #[case] expected: &str) {
    let spell = TokenizeSpell;
    let result = spell.cast(input);
    assert_eq!(result, expected);
}

#[rstest]
#[case("run", "run:VB")]
#[case("fox", "fox:NN")]
#[case("jumps", "jumps:VBZ")]
fn test_pos_tagging_spell(#[case] input: &str, #[case] expected: &str) {
    let mut dictionary = HashMap::new();
    dictionary.insert("run".to_string(), "VB".to_string());
    dictionary.insert("fox".to_string(), "NN".to_string());
    dictionary.insert("jumps".to_string(), "VBZ".to_string());

    let spell = PosTaggingSpell::new(dictionary);
    let result = spell.cast(input);
    assert_eq!(result, expected);
}