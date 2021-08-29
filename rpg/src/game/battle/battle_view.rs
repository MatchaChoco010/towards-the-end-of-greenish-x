use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, select, FutureExt};
use std::time::Duration;

mod background_view;
mod cover_view;
mod damage_number_view;
mod enemy_view;
mod items_window;
mod menu_view;
mod message_window_view;
mod number_view;
mod player_view;
mod skills_window;

use crate::game::battle::battle_model::SelectCommandData;
use crate::game::battle::battle_view::background_view::*;
use crate::game::battle::battle_view::cover_view::*;
use crate::game::battle::battle_view::enemy_view::*;
use crate::game::battle::battle_view::items_window::*;
use crate::game::battle::battle_view::menu_view::*;
use crate::game::battle::battle_view::message_window_view::*;
use crate::game::battle::battle_view::player_view::*;
use crate::game::battle::battle_view::skills_window::*;
use crate::game_data;
use crate::game_data::*;
use crate::input;

pub(super) use items_window::ItemWindowItem;
pub(super) use number_view::Number;
pub(super) use number_view::NumberView;
pub(super) use skills_window::SkillWindowItem;

pub(super) enum BattleCommand {
    Skill(SkillId),
    Item(ItemId),
}

// pub(super) struct PlayerModifierViewItem<'a> {
//     name_key: String,
//     turns: u32,
//     description_key: String,
//     description_args: &'a [&'a str],
// }
// pub(super) struct EnemyModifierViewItem<'a> {
//     name_key: String,
//     turns: u32,
//     description_key: String,
//     description_args: &'a [&'a str],
// }

pub(super) struct BattleView<'a> {
    cx: &'a AnimationEngineContext,
    background: BackgroundView<'a>,
    cover: CoverView<'a>,
    message_window: MessageWindowView<'a>,
    player: PlayerView<'a>,
    enemy: EnemyView<'a>,
    menu: MenuView<'a>,
    skills: SkillsWindow<'a>,
    items: ItemsWindow<'a>,
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
        let items = ItemsWindow::new(cx);
        Self {
            cx,
            background,
            cover,
            message_window,
            player,
            enemy,
            menu,
            skills,
            items,
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

    pub(super) async fn select_command<'b>(
        &mut self,
        data: SelectCommandData<'b>,
    ) -> BattleCommand {
        let SelectCommandData {
            skills,
            skill_data,
            items,
            item_data,
        } = data;

        let mut index = 0;
        self.set_menu_cursor(index);
        self.show_menu().await;

        let command = 'select_command: loop {
            self.set_menu_active(true);
            loop {
                select! {
                    _ = input::wait_down(self.cx).fuse() => {
                        index = (index + 1) % 5;
                        self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    }
                    _ = input::wait_up(self.cx).fuse() => {
                        index = (index + 5 - 1) % 5;
                        self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    }
                    _ = input::wait_select_button(self.cx).fuse() => {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        break;
                    }
                }
                self.set_menu_cursor(index);
                delay(Duration::from_millis(150)).await;
            }

            next_frame().await;
            self.set_menu_active(false);

            match index {
                0 => {
                    select! {
                        _ = self.enemy_blink_animation_loop().fuse() => unreachable!(),
                        _ = input::wait_select_button(self.cx).fuse() => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                            break 'select_command BattleCommand::Skill(SkillId(0));
                        }
                        _ = input::wait_cancel_button(self.cx).fuse() => {
                            self.cx.play_sfx("/audio/sfx/cancel.ogg");
                            self.reset_player_blink();
                            self.reset_enemy_blink();
                        }
                    }
                }
                1 => loop {
                    if let Some(skill) = self.skills.select_skill(&skills, &skill_data).await {
                        if skill.skill_target == SkillTarget::Enemy {
                            select! {
                                _ = self.enemy_blink_animation_loop().fuse() => unreachable!(),
                                _ = input::wait_select_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/select.ogg");
                                    break 'select_command BattleCommand::Skill(skill.id);
                                }
                                _ = input::wait_cancel_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                                    self.reset_player_blink();
                                    self.reset_enemy_blink();
                                }
                            }
                        } else {
                            select! {
                                _ = self.player_blink_animation_loop().fuse() => unreachable!(),
                                _ = input::wait_select_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/select.ogg");
                                    break 'select_command BattleCommand::Skill(skill.id);
                                }
                                _ = input::wait_cancel_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                                    self.reset_player_blink();
                                    self.reset_enemy_blink();
                                }
                            }
                        }
                    } else {
                        break;
                    }
                },
                2 => loop {
                    if let Some(item) = self.items.select_item(&items, &item_data).await {
                        if item.item_target == ItemTarget::Enemy {
                            select! {
                                _ = self.enemy_blink_animation_loop().fuse() => unreachable!(),
                                _ = input::wait_select_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/select.ogg");
                                    break 'select_command BattleCommand::Item(item.id);
                                }
                                _ = input::wait_cancel_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                                    self.reset_player_blink();
                                    self.reset_enemy_blink();
                                }
                            }
                        } else {
                            select! {
                                _ = self.player_blink_animation_loop().fuse() => unreachable!(),
                                _ = input::wait_select_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/select.ogg");
                                    break 'select_command BattleCommand::Item(item.id);
                                }
                                _ = input::wait_cancel_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                                    self.reset_player_blink();
                                    self.reset_enemy_blink();
                                }
                            }
                        }
                    } else {
                        break;
                    }
                },
                3 => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                }
                4 => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                }
                _ => unreachable!(),
            }
        };

        self.reset_player_blink();
        self.reset_enemy_blink();
        self.hide_menu().await;

        command
    }
}
