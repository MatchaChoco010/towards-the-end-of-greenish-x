use animation_engine::*;

use crate::game;
use crate::game::battle::battle_view::*;
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
    view: BattleView<'a>,
}
impl<'a> BattleScene<'a> {
    pub(crate) fn new(
        cx: &'a AnimationEngineContext,
        player_data: &'a game_data::PlayerData,
        player_index: usize,
        item_data: &'a Vec<game_data::ItemData>,
        battle_id: usize,
        time: game_data::BattleTime,
    ) -> Self {
        let view = BattleView::new(cx, time);
        Self {
            cx,
            player_data,
            player_index,
            item_data,
            view,
        }
    }

    pub(crate) async fn start(&self, player_state: &mut game::PlayerState) -> BattleResult {
        self.view.set_monster_image(
            "/image/monster/monster.png",
            "/image/monster/monster-shadow.png",
        );
        self.view.battle_start().await;

        input::wait_select_button(self.cx).await;

        self.view.battle_end().await;
        BattleResult::Win
    }
}
