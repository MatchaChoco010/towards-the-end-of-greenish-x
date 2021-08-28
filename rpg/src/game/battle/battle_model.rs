use crate::game::battle::items_window::*;
use crate::game::battle::number_view::*;
use crate::game::battle::skills_window::*;
use crate::game::PlayerState;
use crate::game_data::*;

pub(super) struct SelectCommandData<'a> {
    pub skills: Vec<SkillWindowItem>,
    pub skill_data: Vec<&'a SkillData>,
    pub items: Vec<ItemWindowItem>,
    pub item_data: Vec<&'a ItemData>,
}

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

    fn get_skill_data(
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

    fn get_item_data(&self, player_state: &PlayerState) -> (Vec<ItemWindowItem>, Vec<&ItemData>) {
        player_state
            .get_items()
            .iter()
            .flat_map(|&(item_id, item_count)| {
                let item_data = self.item_data.iter().find(|&i| i.id == item_id).unwrap();
                let active = true;
                (0..item_count).map(move |_| {
                    (
                        ItemWindowItem {
                            name_key: item_data.item_name.to_owned(),
                            active,
                        },
                        item_data,
                    )
                })
            })
            .unzip::<_, _, Vec<_>, Vec<_>>()
    }

    pub(super) fn select_command_data(&self, player_state: &PlayerState) -> SelectCommandData {
        let (skills, skill_data) = self.get_skill_data(player_state);
        let (items, item_data) = self.get_item_data(player_state);
        SelectCommandData {
            skills,
            skill_data,
            items,
            item_data,
        }
    }
}
