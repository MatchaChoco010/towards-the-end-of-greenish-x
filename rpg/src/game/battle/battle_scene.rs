use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, select, FutureExt};
use std::time::Duration;

use crate::game;
use crate::game::battle::battle_view::*;
use crate::game_data;
use crate::input;

pub enum BattleResult {
    Win,
    Lose,
}

enum BattleCommand {
    Skill(usize),
    Item(usize),
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
        let view = BattleView::new(cx, time, player_index);
        Self {
            cx,
            player_data,
            player_index,
            item_data,
            view,
        }
    }

    async fn select_command(&self, player_state: &mut game::PlayerState) -> BattleCommand {
        let mut index = 0;
        self.view.set_menu_cursor(index);
        self.view.show_menu().await;

        loop {
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

            if index == 2 {
                select! {
                    _ = self.view.player_blink_animation_loop().fuse() => unreachable!(),
                    _ = input::wait_select_button(self.cx).fuse() => {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        break;
                    }
                    _ = input::wait_cancel_button(self.cx).fuse() => {
                        self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    }
                }
            } else {
                select! {
                    _ = self.view.enemy_blink_animation_loop().fuse() => unreachable!(),
                    _ = input::wait_select_button(self.cx).fuse() => {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        break;
                    }
                    _ = input::wait_cancel_button(self.cx).fuse() => {
                        self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    }
                }
            }
            self.view.reset_player_blink();
            self.view.reset_enemy_blink();
        }

        self.view.reset_player_blink();
        self.view.reset_enemy_blink();
        self.view.hide_menu().await;

        if index == 2 {
            BattleCommand::Item(0)
        } else {
            BattleCommand::Skill(0)
        }
    }

    pub(crate) async fn start(&self, player_state: &mut game::PlayerState) -> BattleResult {
        self.view.set_monster_image(
            "/image/monster/monster.png",
            "/image/monster/monster-shadow.png",
        );
        self.view.set_player_hp(150, 150);
        self.view.set_player_tp(50, 50);
        self.view.set_enemy_hp(250, 250);

        self.view.battle_start().await;

        self.view.set_turn_number(1);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.player_blink_animation().await;
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;

        self.view.set_turn_number(2);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(200, 250);
        join!(
            self.view.enemy_damage_animation(50),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "50"]),
        );
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;
        self.view.set_player_hp(140, 150);
        self.view.set_player_tp(50, 50);
        self.view.player_damage_animation(10);
        self.view
            .set_message("battle-message-player-damage", &["10"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(3);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(170, 250);
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
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(4);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.set_enemy_hp(230, 250);
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
        self.view.player_damage_animation(70);
        self.view
            .set_message("battle-message-player-damage", &["70"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(5);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.set_player_hp(90, 150);
        self.view.set_player_tp(30, 50);
        self.view.player_heal_animation(20);
        self.view
            .set_message("battle-message-player-heal", &["20"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.enemy_blink_animation().await;
        self.view.set_player_hp(70, 150);
        self.view.set_player_tp(30, 50);
        self.view.player_damage_animation(20);
        self.view
            .set_message("battle-message-player-damage", &["20"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(6);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(110, 250);
        join!(
            self.view.enemy_damage_animation(120),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "120"]),
        );
        self.view.reset_enemy_blink();
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(10, 150);
        self.view.set_player_tp(7, 50);
        self.view.player_damage_animation(60);
        self.view
            .set_message("battle-message-player-damage", &["60"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(7);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        self.select_command(player_state).await;
        self.view.player_blink_animation().await;
        self.view.set_enemy_hp(0, 250);
        join!(
            self.view.enemy_damage_animation(37680),
            self.view
                .set_message("battle-message-enemy-damage", &["森林チョウ", "37680"]),
        );
        self.view.reset_enemy_blink();
        input::wait_select_button(self.cx).await;
        join!(
            self.view.enemy_down_animation(),
            self.view
                .set_message("battle-message-enemy-down", &["森林チョウ"]),
        );
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(0, 150);
        self.view.set_player_tp(2, 50);
        self.view.player_damage_animation(36724);
        self.view
            .set_message("battle-message-player-damage", &["37624"])
            .await;
        input::wait_select_button(self.cx).await;

        self.view.battle_end().await;
        BattleResult::Win
    }
}
