use animation_engine::*;
use log::info;

use crate::game;
use crate::game_data;

mod battle_scene;

pub use battle_scene::*;

pub async fn battle(
    cx: &AnimationEngineContext,
    player_data: &game_data::PlayerData,
    player_index: usize,
    item_data: &Vec<game_data::ItemData>,
    battle_id: usize,
    player_state: &mut game::PlayerState,
) -> BattleResult {
    info!("Enter Battle Scene!");
    BattleScene::new(cx, player_data, player_index, item_data, battle_id)
        .start(player_state)
        .await
}
