use rune_casting::Casting;
use rune_casting::SkillTreeNode;
use rune_casting::spell::PosTaggingSpell;
use rune_casting::spell::TokenizeSpell;

use std::collections::HashMap;

fn main() {
    // Step 1: 품사 태깅 사전 정의
    let mut dictionary = HashMap::new();
    dictionary.insert("run".to_string(), "VB".to_string());
    dictionary.insert("fox".to_string(), "NN".to_string());
    dictionary.insert("jumps".to_string(), "VBZ".to_string());

    // Step 2: Spell 생성
    let tokenizer = Box::new(TokenizeSpell);
    let pos_tagger = Box::new(PosTaggingSpell::new(dictionary));

    // Step 3: Skill Tree 구성
    let mut root = SkillTreeNode::new(Some(tokenizer));
    root.add_child(SkillTreeNode::new(Some(pos_tagger)));

    // Step 4: Casting 실행
    let casting = Casting::new(root);
    let input = "run fast fox jumps";
    let result = casting.execute(input);

    println!("Skill Tree Parsing Result:\n{}", result);
}
