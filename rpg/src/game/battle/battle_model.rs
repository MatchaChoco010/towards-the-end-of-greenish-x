use crate::game::battle::battle_view::*;
use crate::game::PlayerState;
use crate::game_data::*;

mod battle_state_store;

use battle_state_store::*;

pub(super) struct SelectCommandData<'a> {
    pub skills: Vec<SkillWindowItem>,
    pub skill_data: Vec<&'a SkillData>,
    pub items: Vec<ItemWindowItem>,
    pub item_data: Vec<&'a ItemData>,
}

pub(super) enum BattleCommand {
    Skill(SkillId),
    Item(ItemId),
}

pub(super) enum BattleViewCommand<'a> {
    Message { key: String, args: &'a [&'a str] },
    PlayerBlink,
    EnemyBlink,
    EnemyDamage { damage: i32, hp: i32, max_hp: i32 },
    EnemyHeal { heal: i32, hp: i32, max_hp: i32 },
    EnemyDown,
    PlayerDamage { damage: i32, hp: i32, max_hp: i32 },
    PlayerHeal { heal: i32, hp: i32, max_hp: i32 },
    PlayerSetTp { tp: i32, max_tp: i32 },
    WaitKey,
    Delay { millis: u64 },
}

pub(super) enum BattleTurnResult {
    Win,
    Lose,
    Continue,
}

pub(super) struct BattleModel<'a> {
    player_index: usize,
    player_data: &'a PlayerData,
    item_data: &'a Vec<ItemData>,
    // battle_data
    battle_state: BattleStateStore<'a>,
}
impl<'a> BattleModel<'a> {
    pub(super) fn new(
        player_index: usize,
        player_data: &'a PlayerData,
        item_data: &'a Vec<ItemData>,
    ) -> Self {
        let battle_state = BattleStateStore::new();
        Self {
            player_index,
            player_data,
            item_data,
            battle_state,
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

    pub(super) fn turn_start(&mut self) -> u32 {
        self.battle_state.add("bi-turn-count", 1);
        self.battle_state.get("bi-turn-count") as u32
    }

    pub(super) fn process_turn(
        &mut self,
        player_state: &mut PlayerState,
        command: BattleCommand,
    ) -> (Vec<BattleViewCommand>, BattleTurnResult) {
        let mut view_command = vec![];

        view_command.push(BattleViewCommand::Message {
            key: "test-attack".into(),
            args: &[],
        });
        view_command.push(BattleViewCommand::PlayerSetTp { tp: 35, max_tp: 50 });
        view_command.push(BattleViewCommand::PlayerBlink);
        view_command.push(BattleViewCommand::EnemyDamage {
            damage: 50,
            hp: 200,
            max_hp: 250,
        });
        view_command.push(BattleViewCommand::Delay { millis: 450 });
        view_command.push(BattleViewCommand::EnemyDamage {
            damage: 50,
            hp: 150,
            max_hp: 250,
        });
        view_command.push(BattleViewCommand::Delay { millis: 300 });

        view_command.push(BattleViewCommand::Message {
            key: "test-enemy-attack".into(),
            args: &["森林チョウ"],
        });
        view_command.push(BattleViewCommand::WaitKey);
        view_command.push(BattleViewCommand::EnemyBlink);
        view_command.push(BattleViewCommand::PlayerDamage {
            damage: 40,
            hp: 110,
            max_hp: 150,
        });
        view_command.push(BattleViewCommand::WaitKey);
        view_command.push(BattleViewCommand::Delay { millis: 300 });

        view_command.push(BattleViewCommand::Delay { millis: 300 });

        (view_command, BattleTurnResult::Continue)
    }
}
