use crate::SkillTreeNode;

pub struct Casting {
    root: SkillTreeNode,
}

impl Casting {
    pub fn new(root: SkillTreeNode) -> Self {
        Self { root }
    }

    pub fn execute(&self, input: &str) -> String {
        self.root.cast(input)
    }
}
