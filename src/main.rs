mod rune;
mod spell;
mod casting;

use crate::rune::tokenizer::TokenizerRune;
use crate::rune::pos::PosTaggingRune;
use crate::casting::{Casting, SkillTreeNode};
use std::collections::HashMap;

fn main() {
    // Step 1: 품사 태깅 사전 생성
    let mut dictionary = HashMap::new();
    dictionary.insert("run".to_string(), "VB".to_string());
    dictionary.insert("fox".to_string(), "NN".to_string());
    dictionary.insert("jumps".to_string(), "VBZ".to_string());

    // Step 2: Skill Tree 구성
    // 루트 노드: 토크나이저 Rune
    let tokenizer = Box::new(TokenizerRune);
    let mut root = SkillTreeNode::new(Some(tokenizer));

    // 자식 노드: 품사 태깅 Rune
    let pos_tagger = Box::new(PosTaggingRune::new(dictionary));
    let pos_node = SkillTreeNode::new(Some(pos_tagger));

    // 트리 구조 연결
    root.add_child(pos_node);

    // Step 3: Casting 생성
    let casting = Casting::new(root);

    // Step 4: 입력 문장 실행
    let input = "run fast fox jumps";
    let result = casting.execute(input);

    println!("Skill Tree Parsing Result:\n{}", result);
}
