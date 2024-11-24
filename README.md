# RuneCasting

`RuneCasting` is a flexible text processing library designed around the concepts of `Rune`, `Spell`, and `Skill Tree`. It allows for composable, reusable, and efficient text processing pipelines.

## Features
- **Rune**: Low-level character or word-based operations.
- **Spell**: Sentence-level processing tasks like tokenization and POS tagging.
- **Skill Tree**: Manage the execution order or conditional execution of Spells.

## Example
```rust
use rune_casting::{Casting, SkillTreeNode};
use rune_casting::spell::{TokenizeSpell, PosTaggingSpell};
use std::collections::HashMap;

fn main() {
    let mut dictionary = HashMap::new();
    dictionary.insert("run".to_string(), "VB".to_string());
    dictionary.insert("fox".to_string(), "NN".to_string());

    let tokenizer = TokenizeSpell;
    let pos_tagger = PosTaggingSpell::new(dictionary);

    let mut root = SkillTreeNode::new(Some(Box::new(tokenizer)));
    root.add_child(SkillTreeNode::new(Some(Box::new(pos_tagger))));

    let casting = Casting::new(root);
    let input = "run fast fox";
    let result = casting.execute(input);

    println!("Result:\n{}", result);
}
```

## License
This project is licensed under the MIT License.