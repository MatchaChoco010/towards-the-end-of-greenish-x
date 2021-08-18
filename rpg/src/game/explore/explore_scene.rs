use animation_engine::executor::*;
use animation_engine::*;
use async_recursion::async_recursion;
use futures::{select, FutureExt};
use log::{info, trace};
use rand::distributions::*;
use rand::prelude::*;

use crate::game::explore::*;
use crate::game::*;
use crate::game_data::*;
use crate::input;
use crate::save_data;

pub(super) struct ExploreScene<'a> {
    cx: &'a AnimationEngineContext,
    player_index: usize,
    _frame: WindowFrame<'a>,
    cover: Cover<'a>,
    current_depth: CurrentDepth<'a>,
    background: Background<'a>,
    message_list: MessageList<'a>,
    skill_item_list_window: SkillItemListWindow<'a>,
}
impl<'a> ExploreScene<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext, player_index: usize, max_depth: u32) -> Self {
        let frame = WindowFrame::new(cx);
        let cover = Cover::new(cx);
        let current_depth = CurrentDepth::new(cx, max_depth);
        let background = Background::new(cx);
        let message_list = MessageList::new(cx);
        let skill_item_list_window = SkillItemListWindow::new(cx);
        Self {
            cx,
            player_index,
            _frame: frame,
            cover,
            current_depth,
            background,
            message_list,
            skill_item_list_window,
        }
    }

    async fn wait_move_forward(
        &self,
        player_state: &PlayerState,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
        save_data: &mut save_data::SaveData,
    ) {
        loop {
            select! {
                _ = input::wait_select_button(self.cx).fuse() => break,
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.skill_item_list_window
                        .show_skills_and_items(&player_state, player_data, item_data)
                        .await;
                }
                _ = input::wait_sub_button(self.cx).fuse() => {
                    options::options(self.cx, save_data).await;
                }
            }
            next_frame().await;
        }
    }

    async fn wait_skill_item_list_window_open(
        &self,
        player_state: &PlayerState,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
        save_data: &mut save_data::SaveData,
    ) {
        loop {
            select! {
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.skill_item_list_window
                        .show_skills_and_items(&player_state, player_data, item_data)
                        .await;
                    break;
                }
                _ = input::wait_sub_button(self.cx).fuse() => {
                    options::options(self.cx, save_data).await;
                }
            }
            next_frame().await;
        }
    }

    async fn wait_choice(
        &mut self,
        message: impl ToString,
        choices: &[(u8, &str)],
        player_state: &PlayerState,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
        save_data: &mut save_data::SaveData,
    ) -> usize {
        self.message_list.add_choice_message(message, choices).await;
        loop {
            select! {
                result = self.message_list.wait_choice().fuse() => return result.unwrap(),
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.skill_item_list_window
                        .show_skills_and_items(&player_state, player_data, item_data)
                        .await;
                }
                _ = input::wait_sub_button(self.cx).fuse() => {
                    options::options(self.cx, save_data).await;
                }
            }
            next_frame().await;
        }
    }

    async fn wait_add_skill(
        &self,
        skill_id_list: &[usize],
        player_state: &mut PlayerState,
        player_data: &PlayerData,
    ) {
        self.skill_item_list_window
            .show_add_skill(player_state, player_data, skill_id_list)
            .await;
    }

    #[async_recursion(?Send)]
    async fn process_event(
        &mut self,
        player_state: &mut PlayerState,
        save_data: &mut save_data::SaveData,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
    ) {
        loop {
            self.message_list.add_normal_message("message1").await;
            self.wait_move_forward(player_state, player_data, item_data, save_data)
                .await;

            let skills = player_data.skills.iter().map(|s| s.id).collect::<Vec<_>>();
            self.wait_add_skill(&skills, player_state, player_data)
                .await;
            self.wait_move_forward(player_state, player_data, item_data, save_data)
                .await;
            self.current_depth.increment();
        }

        // let messages = [
        //     "message1", "message2", "message3", "message4", "message5", "message6",
        // ];
        // for message in messages {
        //     self.message_list.add_normal_message(message).await;
        //     self.wait_move_forward(player_state, player_data, item_data, save_data)
        //         .await;
        //     self.current_depth.increment();
        // }

        // self.wait_move_forward(player_state, player_data, item_data, save_data)
        //     .await;
        // self.wait_add_skill(&[0, 1, 2, 3], player_state, player_data)
        //     .await;

        // self.message_list.add_normal_message("message").await;
        // self.wait_move_forward(player_state, player_data, item_data, save_data)
        //     .await;
        // self.message_list
        //     .add_normal_blue_message("explore-tutorial-x-key")
        //     .await;
        // self.wait_skill_item_list_window_open(player_state, player_data, item_data, save_data)
        //     .await;

        // player_state.add_item(0);
        // player_state.add_item(0);
        // player_state.add_item(0);
        // player_state.add_item(0);
        // player_state.add_item(1);
        // player_state.add_item(2);
        // player_state.add_item(3);
        // player_state.add_item(4);
        // player_state.add_skill(0, &player_data.skills);
        // player_state.add_skill(1, &player_data.skills);
        // player_state.add_skill(2, &player_data.skills);
        // player_state.add_skill(3, &player_data.skills);

        // self.message_list.add_normal_message("message").await;

        // self.wait_move_forward(player_state, player_data, item_data, save_data)
        //     .await;
        // self.message_list
        //     .add_normal_blue_message("explore-tutorial-x-key")
        //     .await;
        // self.wait_skill_item_list_window_open(player_state, player_data, item_data, save_data)
        //     .await;

        // self.wait_choice(
        //     "message-choice",
        //     &[
        //         (2, "choice1"),
        //         (1, "choice2"),
        //         (3, "choice3"),
        //         (1, "choice4"),
        //     ],
        //     player_state,
        //     player_data,
        //     item_data,
        //     save_data,
        // )
        // .await;

        // self.background.change_to_afternoon().await;
        // let messages = [
        //     "message1", "message2", "message3", "message4", "message5", "message6",
        // ];
        // for message in messages {
        //     self.message_list.add_normal_message(message).await;
        //     self.wait_move_forward(player_state, player_data, item_data, save_data)
        //         .await;
        //     self.current_depth.increment();
        // }

        // self.cx.play_bgm("battle-0");
        // self.cover.start_battle().await;
        // input::wait_select_button(self.cx).await;
        // self.cx.stop_bgm();
        // self.cover.fade_in().await;

        // self.background.change_to_night().await;
        // self.cx.play_bgm("field-1");
        // let messages = [
        //     "message1", "message2", "message3", "message4", "message5", "message6",
        // ];
        // for message in messages {
        //     self.message_list.add_normal_message(message).await;
        //     self.wait_move_forward(player_state, player_data, item_data, save_data)
        //         .await;
        //     self.current_depth.increment();
        // }
    }

    pub(super) async fn start(&mut self, global_data: &mut game::GlobalData) -> anyhow::Result<()> {
        let game_data = &global_data.game_data;
        let save_data = &mut global_data.save_data;
        let player_data = &game_data.player_data()[self.player_index];
        let item_data = game_data.item_data();

        let mut player_state = PlayerState::new();

        self.cx.play_bgm("field-0");
        self.cover.fade_in().await;

        self.process_event(&mut player_state, save_data, player_data, item_data)
            .await;

        self.cover.fade_out().await;
        Ok(())
    }
}
