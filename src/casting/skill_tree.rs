use crate::rune::Rune;

pub struct SkillTreeNode {
    rune: Option<Box<dyn Rune>>,
    children: Vec<SkillTreeNode>,
}

impl SkillTreeNode {
    pub fn new(rune: Option<Box<dyn Rune>>) -> Self {
        Self {
            rune,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: SkillTreeNode) {
        self.children.push(child);
    }

    pub fn cast(&self, input: &str) -> String {
        let result = if let Some(rune) = &self.rune {
            rune.cast(input)
        } else {
            input.to_string()
        };

        self.children
            .iter()
            .fold(result, |acc, child| child.cast(&acc))
    }
}
