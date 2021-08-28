use animation_engine::*;
use futures::{join, select, FutureExt};

use crate::game::battle::background_view::*;
use crate::game::battle::cover_view::*;
use crate::game::battle::enemy_view::*;
use crate::game::battle::menu_view::*;
use crate::game::battle::message_window_view::*;
use crate::game::battle::player_view::*;
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
    message_window: MessageWindowView<'a>,
    player: PlayerView<'a>,
    enemy: EnemyView<'a>,
    menu: MenuView<'a>,
}
impl<'a> BattleView<'a> {
    pub(super) fn new(
        cx: &'a AnimationEngineContext,
        time: game_data::BattleTime,
        player_index: usize,
    ) -> Self {
        let background = BackgroundView::new(cx, time);
        let cover = CoverView::new(cx);
        let message_window = MessageWindowView::new(cx);
        let player = PlayerView::new(cx, player_index);
        let enemy = EnemyView::new(cx);
        let menu = MenuView::new(cx);
        Self {
            cx,
            background,
            cover,
            message_window,
            player,
            enemy,
            menu,
        }
    }

    pub(super) fn set_monster_image(
        &self,
        image_key: impl ToString,
        image_shadow_key: impl ToString,
    ) {
        self.enemy.set_monster_image(image_key, image_shadow_key);
    }

    pub(super) async fn battle_start(&self) {
        join!(
            self.background.start(),
            self.cover.start_battle(),
            self.message_window.start_battle(),
            self.player.start_battle(),
            self.enemy.start_battle(),
        );
    }
    pub(super) async fn battle_end(&self) {
        self.cover.fade_out().await;
    }

    pub(super) fn set_enemy_hp(&self, hp: i32, max_hp: i32) {
        self.enemy.set_hp(hp, max_hp);
    }
    pub(super) fn set_player_hp(&self, hp: i32, max_hp: i32) {
        self.player.set_hp(hp, max_hp);
    }
    pub(super) fn set_player_tp(&self, tp: i32, max_tp: i32) {
        self.player.set_tp(tp, max_tp);
    }

    // pub(super) async fn enemy_boss_damage_animation(&self, damage: i32) {
    //     self.enemy.damage_animation(damage);
    // }
    pub(super) async fn enemy_damage_animation(&self, damage: i32) {
        self.enemy.damage_animation(damage);
    }
    pub(super) async fn enemy_heal_animation(&self, heal: i32) {
        self.enemy.heal_animation(heal);
    }
    pub(super) fn player_damage_animation(&self, damage: i32) {
        self.player.damage_animation(damage);
    }
    pub(super) fn player_heal_animation(&self, heal: i32) {
        self.player.heal_animation(heal);
    }

    pub(super) async fn enemy_down_animation(&self) {
        self.enemy.down_enemy().await;
    }
    // pub(super) async fn enemy_boss_down_animation(&self) {
    //     self.enemy.down_enemy().await;
    // }

    pub(super) async fn set_message(&self, message_key: impl ToString, message_args: &[&str]) {
        self.message_window
            .add_message(message_key, message_args)
            .await;
    }

    pub(super) fn set_turn_number(&self, turn: u32) {
        self.message_window.set_turns(turn);
    }

    pub(super) async fn enemy_blink_animation(&self) {
        self.enemy.blink_animation().await;
    }
    pub(super) async fn enemy_blink_animation_loop(&self) {
        self.enemy.blink_animation_loop().await;
    }
    pub(super) fn reset_enemy_blink(&self) {
        self.enemy.reset_blink();
    }
    pub(super) async fn player_blink_animation(&self) {
        self.player.blink_animation().await;
    }
    pub(super) async fn player_blink_animation_loop(&self) {
        self.player.blink_animation_loop().await;
    }
    pub(super) fn reset_player_blink(&self) {
        self.player.reset_blink();
    }

    pub(super) fn set_menu_active(&self, active: bool) {
        self.menu.set_active(active);
    }
    pub(super) fn set_menu_cursor(&self, index: usize) {
        self.menu.set_cursor(index);
    }
    pub(super) async fn show_menu(&self) {
        self.menu.show().await;
    }
    pub(super) async fn hide_menu(&self) {
        self.menu.hide().await;
    }

    pub(super) fn set_skills(
        &mut self,
        skills: &[SkillViewItem],
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(super) async fn show_skills(&self) {}
    pub(super) async fn hide_skills(&self) {}

    pub(super) fn set_items(
        &mut self,
        items: &[ItemViewItem],
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(super) async fn show_items(&self) {}
    pub(super) async fn hide_items(&self) {}

    pub(super) fn set_player_info(
        &mut self,
        player_modifiers: &Vec<PlayerModifierViewItem>,
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(super) async fn show_player_info(&self) {}
    pub(super) async fn hide_player_info(&self) {}

    pub(super) fn set_enemy_info(
        &mut self,
        enemy_modifiers: &Vec<EnemyModifierViewItem>,
        top_index: usize,
        cursor_index: usize,
    ) {
    }
    pub(super) async fn show_enemy_info(&self) {}
    pub(super) async fn hide_enemy_info(&self) {}
}
