use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, select, FutureExt};
use std::time::Duration;

use crate::game;
use crate::game::battle::battle_view::*;
use crate::game::battle::number_view::Number;
use crate::game::battle::skills_window::SkillWindowItem;
use crate::game_data::*;
use crate::input;

pub enum BattleResult {
    Win,
    Lose,
}

enum BattleCommand {
    Skill(SkillId),
    Item(ItemId),
}

pub(super) struct BattleScene<'a> {
    cx: &'a AnimationEngineContext,
    player_data: &'a PlayerData,
    player_index: usize,
    item_data: &'a Vec<ItemData>,
    // battle_data
    view: BattleView<'a>,
}
impl<'a> BattleScene<'a> {
    pub(crate) fn new(
        cx: &'a AnimationEngineContext,
        player_data: &'a PlayerData,
        player_index: usize,
        item_data: &'a Vec<ItemData>,
        battle_id: usize,
        time: BattleTime,
    ) -> Self {
        let view = BattleView::new(cx, time, player_index);
        Self {
            cx,
            player_data,
            player_index,
            item_data,
            view,
        }
    }

    async fn select_skill(
        &mut self,
        player_state: &mut game::PlayerState, /*, battle_state */
    ) -> Option<SkillId> {
        let (skills, skill_data) = player_state
            .get_skills()
            .iter()
            .map(|&skill_id| {
                let skill_data = self
                    .player_data
                    .skills
                    .iter()
                    .find(|&s| s.id == skill_id)
                    .unwrap();
                let costs = if let SkillCost::Cost(cost) = skill_data.skill_cost {
                    Number::Number(cost as i32)
                } else {
                    Number::Infinity
                };
                let active = skill_data.skill_cost != SkillCost::Infinity;
                (
                    SkillWindowItem {
                        name_key: skill_data.skill_name.to_owned(),
                        costs,
                        active,
                    },
                    skill_data,
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        self.view.set_skills(&skills);

        let len = skills.len();
        let mut view_top_index = 0;
        let mut cursor_index = 0;
        self.view.set_skills_cursor(
            &skill_data[cursor_index].skill_description,
            view_top_index,
            cursor_index,
        );

        self.view.show_skills().await;
        self.view.set_skills_cursor(
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
            self.view.set_skills_cursor(
                &skill_data[cursor_index].skill_description,
                view_top_index,
                cursor_index,
            );
            delay(Duration::from_millis(150)).await;
        };
        self.view.hide_skills().await;

        if canceled {
            None
        } else {
            Some(skill_data[cursor_index].id)
        }
    }

    async fn select_command(&mut self, player_state: &mut game::PlayerState) -> BattleCommand {
        let mut index = 0;
        self.view.set_menu_cursor(index);
        self.view.show_menu().await;

        let command = 'select_command: loop {
            self.view.set_menu_active(true);
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
                self.view.set_menu_cursor(index);
                delay(Duration::from_millis(150)).await;
            }

            next_frame().await;
            self.view.set_menu_active(false);

            match index {
                0 => {
                    select! {
                        _ = self.view.enemy_blink_animation_loop().fuse() => unreachable!(),
                        _ = input::wait_select_button(self.cx).fuse() => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                            break 'select_command BattleCommand::Skill(SkillId(0));
                        }
                        _ = input::wait_cancel_button(self.cx).fuse() => {
                            self.cx.play_sfx("/audio/sfx/cancel.ogg");
                            self.view.reset_player_blink();
                            self.view.reset_enemy_blink();
                        }
                    }
                }
                1 => loop {
                    if let Some(skill) = self.select_skill(player_state).await {
                        if self
                            .player_data
                            .skills
                            .iter()
                            .find(|s| s.id == skill)
                            .unwrap()
                            .skill_target
                            == SkillTarget::Enemy
                        {
                            select! {
                                _ = self.view.enemy_blink_animation_loop().fuse() => unreachable!(),
                                _ = input::wait_select_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/select.ogg");
                                    break 'select_command BattleCommand::Skill(skill);
                                }
                                _ = input::wait_cancel_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                                    self.view.reset_player_blink();
                                    self.view.reset_enemy_blink();
                                }
                            }
                        } else {
                            select! {
                                _ = self.view.player_blink_animation_loop().fuse() => unreachable!(),
                                _ = input::wait_select_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/select.ogg");
                                    break 'select_command BattleCommand::Skill(skill);
                                }
                                _ = input::wait_cancel_button(self.cx).fuse() => {
                                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                                    self.view.reset_player_blink();
                                    self.view.reset_enemy_blink();
                                }
                            }
                        }
                    } else {
                        break;
                    }
                },
                2 => {
                    select! {
                        _ = self.view.enemy_blink_animation_loop().fuse() => unreachable!(),
                        _ = input::wait_select_button(self.cx).fuse() => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                            break 'select_command BattleCommand::Item(ItemId(0));
                        }
                        _ = input::wait_cancel_button(self.cx).fuse() => {
                            self.cx.play_sfx("/audio/sfx/cancel.ogg");
                            self.view.reset_player_blink();
                            self.view.reset_enemy_blink();
                        }
                    }
                }
                3 => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                }
                4 => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                }
                _ => unreachable!(),
            }
        };

        self.view.reset_player_blink();
        self.view.reset_enemy_blink();
        self.view.hide_menu().await;

        command
    }

    pub(crate) async fn start(&mut self, player_state: &mut game::PlayerState) -> BattleResult {
        self.view.set_monster_image(
            "/image/monster/monster.png",
            "/image/monster/monster-shadow.png",
        );
        self.view.set_player_hp(150, 150);
        self.view.set_player_tp(50, 50);
        self.view.set_enemy_hp(250, 250);

        self.view.battle_start().await;
        self.cx.play_sfx("/audio/sfx/monster-bark-0.ogg");

        self.view.set_turn_number(1);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.player_blink_animation().await;
        self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        input::wait_select_button(self.cx).await;
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.enemy_blink_animation().await;

        self.view.set_turn_number(2);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(200, 250);
        self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        join!(
            self.view.enemy_damage_animation(50),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "50"]),
        );
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;
        self.view.set_player_hp(140, 150);
        self.view.set_player_tp(50, 50);
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.player_damage_animation(10);
        self.view
            .set_message("battle-message-player-damage", &["10"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(3);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(170, 250);
        self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        join!(
            self.view.enemy_damage_animation(30),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "30"]),
        );
        self.view.reset_enemy_blink();
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;
        self.view.set_player_hp(140, 150);
        self.view.set_player_tp(30, 50);
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.player_damage_animation(70);
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(4);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.set_enemy_hp(230, 250);
        self.cx.play_sfx("/audio/sfx/heal.ogg");
        join!(
            self.view.enemy_heal_animation(60),
            self.view
                .set_message("battle-message-enemy-heal", &["森林チョウ", "50"]),
        );
        self.view.reset_enemy_blink();
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;
        self.view.set_player_hp(70, 150);
        self.view.set_player_tp(30, 50);
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.player_damage_animation(70);
        self.view
            .set_message("battle-message-player-damage", &["70"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(5);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.set_player_hp(90, 150);
        self.view.set_player_tp(30, 50);
        self.cx.play_sfx("/audio/sfx/heal.ogg");
        self.view.player_heal_animation(20);
        self.view
            .set_message("battle-message-player-heal", &["20"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;
        self.view.set_player_hp(70, 150);
        self.view.set_player_tp(30, 50);
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.player_damage_animation(20);
        self.view
            .set_message("battle-message-player-damage", &["20"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(6);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(110, 250);
        self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        join!(
            self.view.enemy_damage_animation(120),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "120"]),
        );
        self.view.reset_enemy_blink();
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(10, 150);
        self.view.set_player_tp(7, 50);
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.player_damage_animation(60);
        self.view
            .set_message("battle-message-player-damage", &["60"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(7);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        match self.select_command(player_state).await {
            BattleCommand::Skill(_skill) => (),
            BattleCommand::Item(_item) => (),
        }
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(0, 250);
        self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        join!(
            self.view.enemy_damage_animation(37680),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "37680"]),
        );
        self.view.reset_enemy_blink();
        input::wait_select_button(self.cx).await;
        self.cx.play_sfx("/audio/sfx/down.ogg");
        join!(
            self.view.enemy_down_animation(),
            self.view
                .set_message("battle-message-enemy-down", &["森林チョウ"]),
        );
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(0, 150);
        self.view.set_player_tp(2, 50);
        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        self.view.player_damage_animation(36724);
        self.view
            .set_message("battle-message-player-damage", &["37624"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.battle_end().await;
        BattleResult::Win
    }
}
