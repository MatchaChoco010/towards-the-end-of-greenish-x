use animation_engine::*;
use log::info;

use crate::game::game;

mod background;
mod confirm_get_skill_window;
mod confirm_override_skill_window;
mod cover;
mod current_depth;
mod explore_scene;
mod message_list;
mod skill_item_list_window;
mod window_frame;

use background::*;
use confirm_get_skill_window::*;
use confirm_override_skill_window::*;
use cover::*;
use current_depth::*;
use explore_scene::*;
use message_list::*;
use skill_item_list_window::*;
use window_frame::*;

pub use explore_scene::ExploreResult;

pub async fn explore(
    cx: &AnimationEngineContext,
    global_data: &mut game::GlobalData,
    player_index: usize,
) -> ExploreResult {
    info!("Enter Explore Scene!");
    ExploreScene::new(cx, player_index).start(global_data).await
}
