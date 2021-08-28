use crate::game::battle::number_view::*;
use crate::game::battle::skills_window::*;
use crate::game::PlayerState;
use crate::game_data::*;

pub(super) struct BattleModel<'a> {
    player_index: usize,
    player_data: &'a PlayerData,
    item_data: &'a Vec<ItemData>,
    // battle_data
    // battle_state
}
impl<'a> BattleModel<'a> {
    pub(super) fn new(
        player_index: usize,
        player_data: &'a PlayerData,
        item_data: &'a Vec<ItemData>,
    ) -> Self {
        Self {
            player_index,
            player_data,
            item_data,
        }
    }

    pub(super) fn get_skill_data(
        &self,
        player_state: &PlayerState,
    ) -> (Vec<SkillWindowItem>, Vec<&SkillData>) {
        player_state
            .get_skills()
            .iter()
            .map(|&skill_id| {
                let skill_data = self
                    .player_data
                    .skills
                    .iter()
                    .find(|&s| s.id == skill_id)
                    .unwrap();
                let costs = if let SkillCost::Cost(cost) = skill_data.skill_cost {
                    Number::Number(cost as i32)
                } else {
                    Number::Infinity
                };
                let active = skill_data.skill_cost != SkillCost::Infinity;
                (
                    SkillWindowItem {
                        name_key: skill_data.skill_name.to_owned(),
                        costs,
                        active,
                    },
                    skill_data,
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>()
    }
}
