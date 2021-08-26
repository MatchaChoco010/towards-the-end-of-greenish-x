use animation_engine::executor::next_frame;
use animation_engine::*;
use futures::{join, select, FutureExt};

use crate::game;
use crate::game::battle::battle_view::*;
use crate::game_data;
use crate::input;

pub enum BattleResult {
    Win,
    Lose,
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

    pub(crate) async fn start(&self, player_state: &mut game::PlayerState) -> BattleResult {
        self.view.set_monster_image(
            "/image/monster/monster.png",
            "/image/monster/monster-shadow.png",
        );
        self.view.set_player_hp(150, 150);
        self.view.set_player_tp(50, 50);

        self.view.battle_start().await;

        self.view.set_turn_number(1);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.player_blink_animation().await;

        self.view.set_turn_number(2);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.player_blink_animation().await;
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(140, 150);
        self.view.set_player_tp(50, 50);
        self.view.player_damage_animation(10);
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(3);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.player_blink_animation().await;
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(140, 150);
        self.view.set_player_tp(30, 50);
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(4);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.player_blink_animation().await;
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(70, 150);
        self.view.set_player_tp(30, 50);
        self.view.player_damage_animation(70);
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(5);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        next_frame().await;
        select! {
            _ = self.view.player_blink_animation_loop().fuse() => unreachable!(),
            _ = input::wait_select_button(self.cx).fuse() => (),
        }
        self.view.set_player_hp(90, 150);
        self.view.set_player_tp(30, 50);
        self.view.player_heal_animation(20);
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(6);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.player_blink_animation().await;
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(10, 150);
        self.view.set_player_tp(7, 50);
        self.view.player_damage_animation(80);
        input::wait_select_button(self.cx).await;

        self.view.set_turn_number(7);
        self.view
            .set_message("battle-message-battle-start", &["森林チョウ"])
            .await;
        input::wait_select_button(self.cx).await;
        self.view.player_blink_animation().await;
        input::wait_select_button(self.cx).await;
        self.view.set_player_hp(0, 150);
        self.view.set_player_tp(2, 50);
        self.view.player_damage_animation(36724);
        input::wait_select_button(self.cx).await;

        self.view.battle_end().await;
        BattleResult::Win
    }
}
