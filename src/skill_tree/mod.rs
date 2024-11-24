use crate::spell::Spell;

pub struct SkillTreeNode {
    spell: Option<Box<dyn Spell>>, // 각 노드에서 실행할 Spell
    children: Vec<SkillTreeNode>, // 자식 노드들
}

impl SkillTreeNode {
    pub fn new(spell: Option<Box<dyn Spell>>) -> Self {
        Self {
            spell,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: SkillTreeNode) {
        self.children.push(child);
    }

    pub fn cast(&self, input: &str) -> String {
        let result = if let Some(spell) = &self.spell {
            spell.cast(input)
        } else {
            input.to_string()
        };

        self.children
            .iter()
            .fold(result, |acc, child| child.cast(&acc))
    }
}
