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
    current_explore_bgm: String,
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
            current_explore_bgm: "field-0".into(),
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
        rng: &mut ThreadRng,
        player_state: &mut PlayerState,
        save_data: &mut save_data::SaveData,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
        level_item: &LevelItem,
    ) {
        match level_item {
            LevelItem::Sequence { items } => {
                for level_item in items.iter() {
                    self.process_event(
                        rng,
                        player_state,
                        save_data,
                        player_data,
                        item_data,
                        level_item,
                    )
                    .await;
                }
            }
            LevelItem::Random { branches } => {
                let dist = WeightedIndex::new(branches.iter().map(|b| b.weight)).unwrap();
                let index = dist.sample(rng);

                trace!("Random level index: {}", index);

                let level_item = &branches[index].item;
                self.process_event(
                    rng,
                    player_state,
                    save_data,
                    player_data,
                    item_data,
                    level_item,
                )
                .await;
            }
            LevelItem::Message {
                text,
                blue,
                no_weight,
            } => {
                if blue.is_some() && blue.unwrap() {
                    self.message_list.add_normal_blue_message(text).await;
                } else {
                    self.message_list.add_normal_message(text).await;
                }
                if !(no_weight.is_some() && no_weight.unwrap()) {
                    self.wait_move_forward(player_state, player_data, item_data, save_data)
                        .await;
                }
            }
            LevelItem::Choice { text, branches } => {
                let choices = &branches
                    .iter()
                    .map(|b| (b.text_lines, b.text.as_str()))
                    .collect::<Vec<_>>();
                let index = self
                    .wait_choice(
                        text,
                        choices,
                        player_state,
                        player_data,
                        item_data,
                        save_data,
                    )
                    .await;

                trace!("Choice level index: {}", index);

                self.process_event(
                    rng,
                    player_state,
                    save_data,
                    player_data,
                    item_data,
                    &branches[index].item,
                )
                .await;
            }
            LevelItem::PlayBGM { bgm } => {
                self.cx.play_bgm(bgm);
                self.current_explore_bgm = bgm.to_string();
            }
            LevelItem::StopBGM => self.cx.stop_bgm(),
            LevelItem::ResumeOrPlayBGM { bgm } => {
                self.cx.resume_or_play_bgm(bgm);
                self.current_explore_bgm = bgm.to_string();
            }
            LevelItem::ChangeToAfternoon => self.background.change_to_afternoon().await,
            LevelItem::ChangeToNight => self.background.change_to_night().await,
            LevelItem::Battle { id: _id } => {
                self.cx.play_bgm("battle-0");
                self.cover.start_battle().await;
                input::wait_select_button(self.cx).await;
                self.cx.resume_or_play_bgm(&self.current_explore_bgm);
                self.cover.fade_in().await;
            }
            LevelItem::WaitOpenSkillItemList => {
                self.wait_skill_item_list_window_open(
                    player_state,
                    player_data,
                    item_data,
                    save_data,
                )
                .await
            }
            LevelItem::GetSkill { skills, count } => {
                let mut list = skills
                    .iter()
                    .flat_map(|sr| {
                        let rarity = sr.rarity;
                        let weight = sr.weight;
                        player_data
                            .skills
                            .iter()
                            .filter(move |s| s.rarity == rarity)
                            .map(move |s| (s.id, s.rarity_weight * weight))
                    })
                    .filter(|(s, _)| !player_state.get_skills().contains(s))
                    .collect::<Vec<_>>();
                let mut candidate_skills = vec![];
                for _ in 0..(list.len().min(*count as usize)) {
                    let dist = WeightedIndex::new(list.iter().map(|(_, w)| w)).unwrap();
                    let index = dist.sample(rng);
                    let (skill_id, _) = list.remove(index);
                    candidate_skills.push(skill_id);
                }
                self.wait_add_skill(&candidate_skills, player_state, player_data)
                    .await;
            }
            LevelItem::AddItem { item_id, count } => {
                for _ in 0..*count {
                    player_state.add_item(*item_id);
                }
            }
        }
    }

    pub(super) async fn start(&mut self, global_data: &mut game::GlobalData) -> anyhow::Result<()> {
        let rng = &mut *global_data.rng.borrow_mut();
        let game_data = &global_data.game_data;
        let save_data = &mut global_data.save_data;
        let player_data = &game_data.player_data()[self.player_index];
        let item_data = game_data.item_data();
        let level_data = game_data.level_data();

        let mut player_state = PlayerState::new();
        let player_state = &mut player_state;

        self.cx.play_bgm(&self.current_explore_bgm);
        self.cover.fade_in().await;

        for LevelData {
            item: level_item,
            index,
        } in level_data.iter()
        {
            info!("Level index: {}", index);

            self.process_event(
                rng,
                player_state,
                save_data,
                player_data,
                item_data,
                level_item,
            )
            .await;

            self.current_depth.increment();
            self.message_list.add_space().await;
        }

        self.cover.fade_out().await;
        Ok(())
    }
}
