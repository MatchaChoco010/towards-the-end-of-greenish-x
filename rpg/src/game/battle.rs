use animation_engine::*;
use log::info;

use crate::game;
use crate::game_data;

mod background_view;
mod battle_scene;
mod battle_view;
mod cover_view;
mod damage_number_view;
mod enemy_view;
mod message_window_view;
mod number_view;
mod player_view;

pub use battle_scene::*;

pub async fn battle(
    cx: &AnimationEngineContext,
    player_data: &game_data::PlayerData,
    player_index: usize,
    item_data: &Vec<game_data::ItemData>,
    battle_id: usize,
    player_state: &mut game::PlayerState,
    time: game_data::BattleTime,
) -> BattleResult {
    info!("Enter Battle Scene!");
    BattleScene::new(cx, player_data, player_index, item_data, battle_id, time)
        .start(player_state)
        .await
}
