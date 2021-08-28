use animation_engine::*;
use futures::join;

use crate::game;
use crate::game::battle::battle_model::*;
use crate::game::battle::battle_view::*;
use crate::game_data::*;
use crate::input;

pub enum BattleResult {
    Win,
    Lose,
}

pub(super) struct BattleScene<'a> {
    cx: &'a AnimationEngineContext,
    view: BattleView<'a>,
    model: BattleModel<'a>,
}
impl<'a> BattleScene<'a> {
    pub(crate) fn new(
        cx: &'a AnimationEngineContext,
        player_index: usize,
        player_data: &'a PlayerData,
        item_data: &'a Vec<ItemData>,
        // battle_data: ,
        time: BattleTime,
    ) -> Self {
        let view = BattleView::new(cx, time, player_index);
        let model = BattleModel::new(player_index, player_data, item_data);
        Self { cx, view, model }
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

        // self.view
        //     .set_message("battle-message-turn-start", &[])
        //     .await;
        // match self.select_command(player_state).await {
        //     BattleCommand::Skill(_skill) => (),
        //     BattleCommand::Item(_item) => (),
        // }
        // self.view.player_blink_animation().await;
        // self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        // input::wait_select_button(self.cx).await;
        // self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        // self.view.enemy_blink_animation().await;

        // self.view.set_turn_number(2);
        // self.view
        //     .set_message("battle-message-turn-start", &[])
        //     .await;
        // match self.select_command(player_state).await {
        //     BattleCommand::Skill(_skill) => (),
        //     BattleCommand::Item(_item) => (),
        // }
        // self.view.player_blink_animation().await;
        // self.view.set_enemy_hp(200, 250);
        // self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        // join!(
        //     self.view.enemy_damage_animation(50),
        //     self.view
        //         .set_message("battle-message-enemy-damage", &["森林チョウ", "50"]),
        // );
        // input::wait_select_button(self.cx).await;
        // self.view.enemy_blink_animation().await;
        // self.view.set_player_hp(140, 150);
        // self.view.set_player_tp(50, 50);
        // self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        // self.view.player_damage_animation(10);
        // self.view
        //     .set_message("battle-message-player-damage", &["10"])
        //     .await;
        // input::wait_select_button(self.cx).await;

        // self.view.set_turn_number(3);
        // self.view
        //     .set_message("battle-message-turn-start", &[])
        //     .await;
        // match self.select_command(player_state).await {
        //     BattleCommand::Skill(_skill) => (),
        //     BattleCommand::Item(_item) => (),
        // }
        // self.view.player_blink_animation().await;
        // self.view.set_enemy_hp(170, 250);
        // self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        // join!(
        //     self.view.enemy_damage_animation(30),
        //     self.view
        //         .set_message("battle-message-enemy-damage", &["森林チョウ", "30"]),
        // );
        // self.view.reset_enemy_blink();
        // input::wait_select_button(self.cx).await;
        // self.view.enemy_blink_animation().await;
        // self.view.set_player_hp(140, 150);
        // self.view.set_player_tp(30, 50);
        // self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        // self.view.player_damage_animation(70);
        // input::wait_select_button(self.cx).await;

        // self.view.set_turn_number(4);
        // self.view
        //     .set_message("battle-message-turn-start", &[])
        //     .await;
        // match self.select_command(player_state).await {
        //     BattleCommand::Skill(_skill) => (),
        //     BattleCommand::Item(_item) => (),
        // }
        // self.view.set_enemy_hp(230, 250);
        // self.cx.play_sfx("/audio/sfx/heal.ogg");
        // join!(
        //     self.view.enemy_heal_animation(60),
        //     self.view
        //         .set_message("battle-message-enemy-heal", &["森林チョウ", "50"]),
        // );
        // self.view.reset_enemy_blink();
        // input::wait_select_button(self.cx).await;
        // self.view.enemy_blink_animation().await;
        // self.view.set_player_hp(70, 150);
        // self.view.set_player_tp(30, 50);
        // self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        // self.view.player_damage_animation(70);
        // self.view
        //     .set_message("battle-message-player-damage", &["70"])
        //     .await;
        // input::wait_select_button(self.cx).await;

        // self.view.set_turn_number(5);
        // self.view
        //     .set_message("battle-message-turn-start", &[])
        //     .await;
        // match self.select_command(player_state).await {
        //     BattleCommand::Skill(_skill) => (),
        //     BattleCommand::Item(_item) => (),
        // }
        // self.view.set_player_hp(90, 150);
        // self.view.set_player_tp(30, 50);
        // self.cx.play_sfx("/audio/sfx/heal.ogg");
        // self.view.player_heal_animation(20);
        // self.view
        //     .set_message("battle-message-player-heal", &["20"])
        //     .await;
        // input::wait_select_button(self.cx).await;
        // self.view.enemy_blink_animation().await;
        // self.view.set_player_hp(70, 150);
        // self.view.set_player_tp(30, 50);
        // self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        // self.view.player_damage_animation(20);
        // self.view
        //     .set_message("battle-message-player-damage", &["20"])
        //     .await;
        // input::wait_select_button(self.cx).await;

        // self.view.set_turn_number(6);
        // self.view
        //     .set_message("battle-message-turn-start", &[])
        //     .await;
        // match self.select_command(player_state).await {
        //     BattleCommand::Skill(_skill) => (),
        //     BattleCommand::Item(_item) => (),
        // }
        // self.view.player_blink_animation().await;
        // self.view.set_enemy_hp(110, 250);
        // self.cx.play_sfx("/audio/sfx/hit-0.ogg");
        // join!(
        //     self.view.enemy_damage_animation(120),
        //     self.view
        //         .set_message("battle-message-enemy-damage", &["森林チョウ", "120"]),
        // );
        // self.view.reset_enemy_blink();
        // input::wait_select_button(self.cx).await;
        // self.view.set_player_hp(10, 150);
        // self.view.set_player_tp(7, 50);
        // self.cx.play_sfx("/audio/sfx/hit-1.ogg");
        // self.view.player_damage_animation(60);
        // self.view
        //     .set_message("battle-message-player-damage", &["60"])
        //     .await;
        // input::wait_select_button(self.cx).await;

        // self.view.set_turn_number(7);
        self.view
            .set_message("battle-message-turn-start", &[])
            .await;
        let data = self.model.select_command_data(player_state);
        match self.view.select_command(data).await {
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
