use rstest::rstest;
use rune_casting::rune::{Rune, UppercaseRune};

#[rstest]
#[case("hello", "HELLO")]
#[case("world", "WORLD")]
#[case("rust", "RUST")]
fn test_uppercase_rune(#[case] input: &str, #[case] expected: &str) {
    let rune = UppercaseRune;
    let result = rune.cast(input);
    assert_eq!(result, expected);
}
