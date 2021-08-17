use std::collections::{HashMap, HashSet};

use crate::game_data::*;

pub struct PlayerState {
    owned_item: HashMap<usize, u32>,
    owned_skill: HashSet<usize>,
}
impl PlayerState {
    pub fn new() -> Self {
        Self {
            owned_item: HashMap::new(),
            owned_skill: HashSet::new(),
        }
    }

    pub fn add_item(&mut self, item_id: usize) {
        *self.owned_item.entry(item_id).or_default() += 1;
    }

    pub fn use_item(&mut self, item_id: usize) -> anyhow::Result<()> {
        let items = self.owned_item.entry(item_id).or_default();
        if *items == 0 {
            Err(anyhow::Error::msg(format!(
                "Item not owned. item id: {}",
                item_id
            )))
        } else {
            *items -= 1;
            Ok(())
        }
    }

    pub fn get_items(&self) -> Vec<(usize, u32)> {
        self.owned_item
            .iter()
            .map(|(&id, &count)| (id, count))
            .collect()
    }

    pub fn is_skill_override(&self, skill_id: usize, skill_list: &Vec<SkillData>) -> Option<usize> {
        let skill_type = skill_list
            .iter()
            .find(|s| s.id == skill_id)
            .unwrap()
            .skill_type;
        for skill in self.owned_skill.iter() {
            let ty = skill_list
                .iter()
                .find(|s| &s.id == skill)
                .unwrap()
                .skill_type;
            if skill_type == ty {
                return Some(*skill);
            }
        }
        None
    }

    pub fn add_skill(&mut self, skill_id: usize, skill_list: &Vec<SkillData>) {
        let override_skill = self.is_skill_override(skill_id, skill_list);
        if let Some(skill) = override_skill {
            self.owned_skill.remove(&skill);
        }
        self.owned_skill.insert(skill_id);
    }

    pub fn get_skills(&self) -> Vec<usize> {
        self.owned_skill.iter().map(|&id| id).collect()
    }
}
