use animation_engine::*;
use futures::{join, select, FutureExt};

use crate::game::battle::background_view::*;
use crate::game::battle::cover_view::*;
use crate::game::game;
use crate::game_data;

pub(super) struct SkillViewItem {
    skill_name_key: String,
    skill_description_key: String,
}
pub(super) struct ItemViewItem {
    item_name_key: String,
    item_description_key: String,
}
pub(super) struct PlayerModifierViewItem<'a> {
    name_key: String,
    turns: u32,
    description_key: String,
    description_args: &'a [&'a str],
}
pub(super) struct EnemyModifierViewItem<'a> {
    name_key: String,
    turns: u32,
    description_key: String,
    description_args: &'a [&'a str],
}

pub(super) struct BattleView<'a> {
    cx: &'a AnimationEngineContext,
    background: BackgroundView<'a>,
    cover: CoverView<'a>,
}
impl<'a> BattleView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext, time: game_data::BattleTime) -> Self {
        let background = BackgroundView::new(cx, time);
        let cover = CoverView::new(cx);
        Self {
            cx,
            background,
            cover,
        }
    }

    pub(crate) fn set_monster_image(
        &self,
        image_key: impl ToString,
        image_shadow_key: impl ToString,
    ) {
    }

    pub(crate) async fn battle_start(&self) {
        join!(self.cover.start_battle(), self.background.start());
    }
    pub(crate) async fn battle_end(&self) {
        self.cover.fade_out().await;
    }

    pub(crate) fn set_enemy_hp(&self, hp: i32, max_hp: i32) {}
    pub(crate) fn set_player_hp(&self, hp: i32, max_hp: i32) {}
    pub(crate) fn set_player_tp(&self, tp: i32, max_tp: i32) {}

    pub(crate) async fn enemy_damage_animation(&self, damage: i32, anim_key: impl ToString) {}
    pub(crate) async fn player_damage_animation(&self, damage: i32) {}

    pub(crate) async fn enemy_down_animation(&self, anim_key: impl ToString) {}

    pub(crate) async fn set_message(&self, message_key: impl ToString, message_args: &[&str]) {}

    pub(crate) fn set_turn_number(&self, turn: u32) {}

    pub(super) async fn enemy_blink_animation(&self) {}
    pub(super) async fn enemy_blink_animation_loop(&self) {}
    pub(super) async fn player_blink_animation(&self) {}
    pub(super) async fn player_blink_animation_loop(&self) {}

    pub(crate) fn set_menu_cursor(&self, index: usize) {}
    pub(crate) async fn show_menu(&self) {}
    pub(crate) async fn hide_menu(&self) {}

    pub(crate) fn set_skills(
        &mut self,
        skills: &[SkillViewItem],
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(crate) async fn show_skills(&self) {}
    pub(crate) async fn hide_skills(&self) {}

    pub(crate) fn set_items(
        &mut self,
        items: &[ItemViewItem],
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(crate) async fn show_items(&self) {}
    pub(crate) async fn hide_items(&self) {}

    pub(crate) fn set_player_info(
        &mut self,
        player_modifiers: &Vec<PlayerModifierViewItem>,
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(crate) async fn show_player_info(&self) {}
    pub(crate) async fn hide_player_info(&self) {}

    pub(crate) fn set_enemy_info(
        &mut self,
        enemy_modifiers: &Vec<EnemyModifierViewItem>,
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(crate) async fn show_enemy_info(&self) {}
    pub(crate) async fn hide_enemy_info(&self) {}
}
