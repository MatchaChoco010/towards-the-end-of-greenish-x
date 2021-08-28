use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, select, FutureExt};
use std::time::Duration;

use crate::game::battle::background_view::*;
use crate::game::battle::cover_view::*;
use crate::game::battle::enemy_view::*;
use crate::game::battle::menu_view::*;
use crate::game::battle::message_window_view::*;
use crate::game::battle::player_view::*;
use crate::game::battle::skills_window::*;
use crate::game_data;
use crate::game_data::*;
use crate::input;

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
    skills: SkillsWindow<'a>,
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
        let skills = SkillsWindow::new(cx);
        Self {
            cx,
            background,
            cover,
            message_window,
            player,
            enemy,
            menu,
            skills,
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

    pub(super) async fn select_skill<'b>(
        &mut self,
        skills: Vec<SkillWindowItem>,
        skill_data: Vec<&'b SkillData>,
    ) -> Option<&'b SkillData> {
        self.skills.set_skills(skills.clone());

        let len = skills.len();
        let mut view_top_index = 0;
        let mut cursor_index = 0;
        self.skills.set_cursor(
            &skill_data[cursor_index].skill_description,
            view_top_index,
            cursor_index,
        );

        self.skills.show().await;
        self.skills.set_cursor(
            &skill_data[cursor_index].skill_description,
            view_top_index,
            cursor_index,
        );
        let canceled = loop {
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len > 0 {
                        cursor_index = (cursor_index - 1 + len) % len;
                    }
                    if cursor_index < view_top_index {
                        view_top_index = cursor_index
                    }
                    if cursor_index > view_top_index + 10 {
                        view_top_index = cursor_index - 10
                    }
                },
                _ = input::wait_down(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len > 0 {
                        cursor_index = (cursor_index + 1 + len) % len;
                    }
                    if cursor_index < view_top_index {
                        view_top_index = cursor_index
                    }
                    if cursor_index > view_top_index + 10 {
                        view_top_index = cursor_index - 10
                    }
                },
                _ = input::wait_select_button(self.cx).fuse() => {
                    if skills[cursor_index].active {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        break false;
                    } else {
                        self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    }
                },
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    break true;
                },
            }
            self.skills.set_cursor(
                &skill_data[cursor_index].skill_description,
                view_top_index,
                cursor_index,
            );
            delay(Duration::from_millis(150)).await;
        };
        self.skills.hide().await;

        if canceled {
            None
        } else {
            Some(skill_data[cursor_index])
        }
    }
}
