use animation_engine::executor::*;
use animation_engine::*;
use futures::join;
use std::time::Duration;

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

        let result = loop {
            let turn = self.model.turn_start();
            self.view.set_turn_number(turn);

            self.view
                .set_message("battle-message-turn-start", &[])
                .await;

            let select_command_data = self.model.select_command_data(player_state);
            let command = self.view.select_command(select_command_data).await;

            let (view_commands, turn_result) = self.model.process_turn(player_state, command);
            for command in view_commands.into_iter() {
                match command {
                    BattleViewCommand::Message { key, args } => {
                        self.view.set_message(key, args).await;
                    }
                    BattleViewCommand::PlayerBlink => self.view.player_blink_animation().await,
                    BattleViewCommand::EnemyBlink => self.view.enemy_blink_animation().await,
                    BattleViewCommand::EnemyDamage { damage, hp, max_hp } => {
                        self.view.set_enemy_hp(hp, max_hp);
                        self.cx.play_sfx("/audio/sfx/hit-0.ogg");
                        self.view.enemy_damage_animation(damage).await;
                    }
                    BattleViewCommand::EnemyHeal { heal, hp, max_hp } => {
                        self.view.set_enemy_hp(hp, max_hp);
                        self.cx.play_sfx("/audio/sfx/heal.ogg");
                        self.view.enemy_heal_animation(heal).await;
                    }
                    BattleViewCommand::EnemyDown => {
                        self.cx.play_sfx("/audio/sfx/down.ogg");
                        join!(
                            self.view.enemy_down_animation(),
                            self.view
                                .set_message("battle-message-enemy-down", &["森林チョウ"]),
                        );
                    }
                    BattleViewCommand::PlayerDamage { damage, hp, max_hp } => {
                        self.view.set_player_hp(hp, max_hp);
                        self.cx.play_sfx("/audio/sfx/hit-1.ogg");
                        self.view.player_damage_animation(damage);
                    }
                    BattleViewCommand::PlayerHeal { heal, hp, max_hp } => {
                        self.view.set_player_hp(hp, max_hp);
                        self.cx.play_sfx("/audio/sfx/heal.ogg");
                        self.view.player_heal_animation(heal);
                    }
                    BattleViewCommand::PlayerSetTp { tp, max_tp } => {
                        self.view.set_player_tp(tp, max_tp);
                    }
                    BattleViewCommand::WaitKey => input::wait_select_button(self.cx).await,
                    BattleViewCommand::Delay { millis } => {
                        delay(Duration::from_millis(millis)).await
                    }
                }
            }
            match turn_result {
                BattleTurnResult::Win => break BattleResult::Win,
                BattleTurnResult::Lose => break BattleResult::Lose,
                BattleTurnResult::Continue => continue,
            }
        };

        self.view.battle_end().await;
        result
    }
}
