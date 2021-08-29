use animation_engine::*;
use log::info;

use crate::game;
use crate::game_data;

mod battle_model;
mod battle_scene;
mod battle_view;
pub use battle_scene::*;

pub async fn battle(
    cx: &AnimationEngineContext,
    player_index: usize,
    player_data: &game_data::PlayerData,
    item_data: &Vec<game_data::ItemData>,
    // battle_data: usize,
    player_state: &mut game::PlayerState,
    time: game_data::BattleTime,
) -> BattleResult {
    info!("Enter Battle Scene!");
    BattleScene::new(cx, player_index, player_data, item_data, time)
        .start(player_state)
        .await
}
