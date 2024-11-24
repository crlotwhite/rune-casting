use rstest::rstest;
use rune_casting::casting::Casting;
use rune_casting::skill_tree::SkillTreeNode;
use rune_casting::spell::{TokenizeSpell, PosTaggingSpell};
use std::collections::HashMap;

#[rstest]
fn test_conditional_skill_tree() {
    // Step 1: 품사 태깅 사전 정의
    let mut dictionary = HashMap::new();
    dictionary.insert("run".to_string(), "VB".to_string());
    dictionary.insert("fox".to_string(), "NN".to_string());

    // Step 2: Spell 생성
    let tokenizer = TokenizeSpell;
    let pos_tagger = PosTaggingSpell::new(dictionary);

    // Step 3: 조건부 Skill Tree 구성
    let mut root = SkillTreeNode::new(Some(Box::new(tokenizer)));
    let mut conditional_node = SkillTreeNode::new(None);
    conditional_node.add_child(SkillTreeNode::new(Some(Box::new(pos_tagger))));
    root.add_child(conditional_node);

    // Step 4: Casting 실행
    let casting = Casting::new(root);
    let input = "run fast fox";
    let result = casting.execute(input);

    assert_eq!(result, "run:VB\nfast:UNKNOWN\nfox:NN");
}
