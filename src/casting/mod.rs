mod skill_tree;

pub use skill_tree::SkillTreeNode;

pub struct Casting {
    root: SkillTreeNode,
}

impl Casting {
    pub fn new(root: SkillTreeNode) -> Self {
        Self { root }
    }

    /// 실행 엔트리포인트
    pub fn execute(&self, input: &str) -> String {
        self.root.cast(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenizerRune;

    #[test]
    fn test_casting() {
        // 간단한 Skill Tree 생성
        let tokenizer = Box::new(TokenizerRune);
        let mut root = SkillTreeNode::new(Some(tokenizer));

        let result = Casting::new(root).execute("The quick brown fox");
        assert_eq!(result, "The\nquick\nbrown\nfox");
    }
}
