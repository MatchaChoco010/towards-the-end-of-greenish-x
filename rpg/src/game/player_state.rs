use std::collections::{HashMap, HashSet};

use crate::game_data::*;

pub struct PlayerState {
    owned_item: HashMap<ItemId, u32>,
    owned_skill: HashSet<SkillId>,
}
impl PlayerState {
    pub fn new() -> Self {
        Self {
            owned_item: HashMap::new(),
            owned_skill: HashSet::new(),
        }
    }

    pub fn add_item(&mut self, item_id: ItemId) {
        *self.owned_item.entry(item_id).or_default() += 1;
    }

    pub fn use_item(&mut self, item_id: ItemId) -> anyhow::Result<()> {
        let items = self.owned_item.entry(item_id).or_default();
        if *items == 0 {
            Err(anyhow::Error::msg(format!(
                "Item not owned. item id: {:?}",
                item_id
            )))
        } else {
            *items -= 1;
            Ok(())
        }
    }

    pub fn get_items(&self) -> Vec<(ItemId, u32)> {
        let mut v = self
            .owned_item
            .iter()
            .map(|(&id, &count)| (id, count))
            .collect::<Vec<_>>();
        v.sort_by_key(|(id, _)| id.0);
        v
    }

    pub fn is_skill_override(
        &self,
        skill_id: SkillId,
        skill_list: &Vec<SkillData>,
    ) -> Option<SkillId> {
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

    pub fn add_skill(&mut self, skill_id: SkillId, skill_list: &Vec<SkillData>) {
        let override_skill = self.is_skill_override(skill_id, skill_list);
        if let Some(skill) = override_skill {
            self.owned_skill.remove(&skill);
        }
        self.owned_skill.insert(skill_id);
    }

    pub fn get_skills(&self) -> Vec<SkillId> {
        let mut v = self.owned_skill.iter().map(|&id| id).collect::<Vec<_>>();
        v.sort_by_key(|s| s.0);
        v
    }
}
