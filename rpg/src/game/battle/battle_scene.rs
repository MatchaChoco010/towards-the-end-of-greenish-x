use animation_engine::*;

use crate::game;
use crate::game_data;
use crate::input;

pub enum BattleResult {
    Win,
    Lose,
}

pub(super) struct BattleScene<'a> {
    cx: &'a AnimationEngineContext,
    player_data: &'a game_data::PlayerData,
    player_index: usize,
    item_data: &'a Vec<game_data::ItemData>,
    // battle_data
}
impl<'a> BattleScene<'a> {
    pub(crate) fn new(
        cx: &'a AnimationEngineContext,
        player_data: &'a game_data::PlayerData,
        player_index: usize,
        item_data: &'a Vec<game_data::ItemData>,
        battle_id: usize,
    ) -> Self {
        Self {
            cx,
            player_data,
            player_index,
            item_data,
        }
    }

    pub(crate) async fn start(&self, player_state: &mut game::PlayerState) -> BattleResult {
        input::wait_select_button(self.cx).await;
        BattleResult::Win
    }
}
