use animation_engine::executor::*;
use animation_engine::*;
use async_recursion::async_recursion;
use futures::{select, try_join, FutureExt};
use log::{info, trace};
use rand::distributions::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

use crate::game::game;
use crate::game::player_state::*;
use crate::game_data::*;
use crate::input;

const MESSAGE_LIST_ITEM_SPACE: f32 = 15.0;
const MESSAGE_CHOICE_ITEM_SPACE: f32 = 5.0;

enum MessageListItem<'a> {
    NormalMessage {
        cx: &'a AnimationEngineContext,
        pos: f32,
        window: Entity,
        text: Entity,
    },
    ChoiceMessage {
        cx: &'a AnimationEngineContext,
        pos: f32,
        window: Entity,
        text: Entity,
        choice_texts: Vec<(u8, Entity)>,
        choice_windows: Vec<Entity>,
        choice_window_highlights: Vec<Entity>,
        choice_window_heights: Vec<f32>,
        cursor: Entity,
    },
}
impl<'a> MessageListItem<'a> {
    fn new_normal(cx: &'a AnimationEngineContext, message: impl ToString) -> Self {
        trace!("NormalMessage: {}", message.to_string());
        let window = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-11.png".into(),
            z: 40,
            ..Default::default()
        });
        let text = cx.add_text(AddTextInfo {
            key: message.to_string(),
            font_size: 24.0,
            z: 50,
            ..Default::default()
        });
        Self::NormalMessage {
            cx,
            pos: 0.0,
            window,
            text,
        }
    }

    fn new_choice(
        cx: &'a AnimationEngineContext,
        message: impl ToString,
        choices: &[(u8, impl ToString)],
    ) -> Self {
        trace!(
            "ChoiceMessage: {}, choices: {:?}",
            message.to_string(),
            choices
                .iter()
                .map(|(lines, item)| (lines, item.to_string()))
                .collect::<Vec<_>>()
        );
        let window = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-11.png".into(),
            z: 40,
            ..Default::default()
        });
        let text = cx.add_text(AddTextInfo {
            key: message.to_string(),
            font_size: 24.0,
            z: 50,
            ..Default::default()
        });
        let mut choice_texts = vec![];
        let mut choice_windows = vec![];
        let mut choice_window_highlights = vec![];
        let mut choice_window_heights = vec![];
        for (lines, choice) in choices {
            let text = cx.add_text(AddTextInfo {
                key: choice.to_string(),
                font_size: 24.0,
                z: 55,
                ..Default::default()
            });
            let (window_img, window_highlight_img, height) = match lines {
                1 => (
                    "/image/ui/explore-part-6.png",
                    "/image/ui/explore-part-7.png",
                    56.0,
                ),
                2 => (
                    "/image/ui/explore-part-4.png",
                    "/image/ui/explore-part-5.png",
                    92.0,
                ),
                3 => (
                    "/image/ui/explore-part-8.png",
                    "/image/ui/explore-part-9.png",
                    122.0,
                ),
                _ => panic!("No more than four lines of choices are supported."),
            };
            let window = cx.add_image(AddImageInfo {
                name: window_img.into(),
                z: 40,
                ..Default::default()
            });
            let window_highlight = cx.add_image(AddImageInfo {
                name: window_highlight_img.into(),
                z: 50,
                a: 0.0,
                ..Default::default()
            });
            choice_texts.push((*lines, text));
            choice_windows.push(window);
            choice_window_highlights.push(window_highlight);
            choice_window_heights.push(height);
        }
        let cursor = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-10.png".into(),
            z: 40,
            a: 0.0,
            ..Default::default()
        });
        Self::ChoiceMessage {
            cx,
            pos: 0.0,
            window,
            text,
            choice_texts,
            choice_windows,
            choice_window_highlights,
            choice_window_heights,
            cursor,
        }
    }

    fn set_pos(&mut self, new_pos: f32) {
        match self {
            MessageListItem::NormalMessage {
                cx,
                window,
                text,
                pos,
            } => {
                let x = 170.0 + new_pos * 0.1763269807;
                cx.set_position(*window, x, 720.0 - new_pos, 40)
                    .expect("Failed to set message window position");
                cx.set_position(*text, x + 30.0, 720.0 - new_pos + 25.0, 50)
                    .expect("Failed to set message window position");
                *pos = new_pos;
            }
            MessageListItem::ChoiceMessage {
                cx,
                window,
                text,
                pos,
                choice_texts,
                choice_windows,
                choice_window_heights,
                choice_window_highlights,
                ..
            } => {
                *pos = new_pos;
                let x = 170.0 + *pos * 0.1763269807;
                cx.set_position(*window, x, 720.0 - new_pos, 40)
                    .expect("Failed to set message window position");
                cx.set_position(*text, x + 30.0, 720.0 - new_pos + 25.0, 50)
                    .expect("Failed to set message window position");
                let mut pos = new_pos - 91.0;
                for (i, h) in choice_window_heights.iter().enumerate() {
                    pos = pos - MESSAGE_CHOICE_ITEM_SPACE;
                    let x = 350.0 + pos * 0.1763269807;
                    cx.set_position(
                        choice_windows[i],
                        x - choice_texts[i].0 as f32 * 6.0,
                        720.0 - pos,
                        40,
                    )
                    .expect("Failed to set message window position");
                    cx.set_position(
                        choice_window_highlights[i],
                        x - choice_texts[i].0 as f32 * 6.0,
                        720.0 - pos,
                        40,
                    )
                    .expect("Failed to set message window position");
                    cx.set_position(
                        choice_texts[i].1,
                        x + 30.0,
                        720.0 - pos + 15.0 + choice_texts[i].0 as f32 * 5.0,
                        50,
                    )
                    .expect("Failed to set message window position");
                    pos = pos - h;
                }
            }
        }
    }

    fn get_pos(&self) -> f32 {
        match self {
            MessageListItem::NormalMessage { pos, .. } => *pos,
            MessageListItem::ChoiceMessage { pos, .. } => *pos,
        }
    }

    fn change_opacity(&self) {
        match self {
            MessageListItem::NormalMessage {
                cx, window, text, ..
            } => {
                cx.set_opacity(*window, 0.6)
                    .expect("Failed to set message window opacity");
                cx.set_opacity(*text, 0.6)
                    .expect("Failed to set message window opacity");
            }
            MessageListItem::ChoiceMessage {
                cx,
                window,
                text,
                choice_texts,
                choice_windows,
                choice_window_heights,
                ..
            } => {
                cx.set_opacity(*window, 0.6)
                    .expect("Failed to set message window opacity");
                cx.set_opacity(*text, 0.6)
                    .expect("Failed to set message window opacity");
                for (i, _) in choice_window_heights.iter().enumerate() {
                    cx.set_opacity(choice_windows[i], 0.6)
                        .expect("Failed to set message window opacity");
                    cx.set_opacity(choice_texts[i].1, 0.6)
                        .expect("Failed to set message window opacity");
                }
            }
        }
    }

    fn get_height(&self) -> f32 {
        match self {
            MessageListItem::NormalMessage { .. } => 91.0 + MESSAGE_LIST_ITEM_SPACE,
            MessageListItem::ChoiceMessage {
                choice_window_heights,
                ..
            } => {
                91.0 + MESSAGE_LIST_ITEM_SPACE
                    + choice_window_heights
                        .iter()
                        .fold(0.0, |acc, h| acc + h + MESSAGE_CHOICE_ITEM_SPACE)
            }
        }
    }

    async fn choice(&self) -> Option<usize> {
        match self {
            MessageListItem::NormalMessage { .. } => None,
            MessageListItem::ChoiceMessage {
                cx,
                pos,
                choice_texts,
                choice_window_heights,
                choice_window_highlights,
                cursor,
                ..
            } => {
                cx.set_opacity(*cursor, 1.0).unwrap();
                let len = choice_window_heights.len();
                let mut index = 0;
                loop {
                    for (i, &entity) in choice_window_highlights.iter().enumerate() {
                        if i == index {
                            cx.set_opacity(entity, 1.0).unwrap();
                        } else {
                            cx.set_opacity(entity, 0.0).unwrap();
                        }
                    }
                    let mut cursor_pos = pos - 91.0;
                    for height in choice_window_heights[0..index].iter() {
                        cursor_pos = cursor_pos - height - MESSAGE_CHOICE_ITEM_SPACE;
                    }
                    match choice_texts[index].0 {
                        2 => cursor_pos = cursor_pos - 18.0,
                        3 => cursor_pos = cursor_pos - 33.0,
                        _ => (),
                    }
                    let x = 310.0 + cursor_pos * 0.1763269807;
                    cx.set_position(*cursor, x, 720.0 - cursor_pos, 50).unwrap();
                    select! {
                        _ = input::wait_up(cx).fuse() => {
                            index = (index - 1 + len) % len;
                            cx.play_sfx("/audio/sfx/cursor.ogg");
                            delay(Duration::from_millis(150)).await;
                        }
                        _ = input::wait_down(cx).fuse() => {
                            index = (index + 1 + len) % len;
                            cx.play_sfx("/audio/sfx/cursor.ogg");
                            delay(Duration::from_millis(150)).await;
                        }
                        _ = input::wait_select_button(cx).fuse() => {
                            cx.play_sfx("/audio/sfx/select.ogg");
                            cx.set_opacity(*cursor, 0.0).unwrap();
                            for (i, &highlight) in choice_window_highlights.iter().enumerate() {
                                if i == index {
                                    cx.set_opacity(highlight, 0.6).unwrap();
                                } else {
                                    cx.set_opacity(highlight, 0.0).unwrap();
                                }
                            }
                            return Some(index)
                        }
                    }
                }
            }
        }
    }
}
impl<'a> Drop for MessageListItem<'a> {
    fn drop(&mut self) {
        match self {
            MessageListItem::NormalMessage {
                cx, window, text, ..
            } => {
                cx.delete_entity(*window);
                cx.delete_entity(*text);
            }
            MessageListItem::ChoiceMessage {
                cx,
                window,
                text,
                choice_texts,
                choice_windows,
                choice_window_highlights,
                cursor,
                ..
            } => {
                cx.delete_entity(*window);
                cx.delete_entity(*text);
                for (_, text) in choice_texts {
                    cx.delete_entity(*text);
                }
                for window in choice_windows {
                    cx.delete_entity(*window);
                }
                for highlight in choice_window_highlights {
                    cx.delete_entity(*highlight);
                }
                cx.delete_entity(*cursor);
            }
        }
    }
}

struct MessageList<'a> {
    cx: &'a AnimationEngineContext,
    items: VecDeque<MessageListItem<'a>>,
}
impl<'a> MessageList<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        let items = VecDeque::new();
        Self { cx, items }
    }

    async fn add_normal_message(&mut self, message: impl ToString) {
        self.cx.play_sfx("/audio/sfx/cursor.ogg");
        self.items.iter().for_each(|item| item.change_opacity());

        let message = MessageListItem::new_normal(self.cx, message);
        let height = message.get_height();
        self.items.push_front(message);
        if self.items.len() > 9 {
            let _ = self.items.pop_back();
        }

        for _ in 0..30 {
            for item in self.items.iter_mut() {
                item.set_pos(item.get_pos() + height / 30.0);
            }
            next_frame().await;
        }
    }

    async fn add_choice_message(
        &mut self,
        message: impl ToString,
        choices: &[(u8, impl ToString)],
    ) {
        self.cx.play_sfx("/audio/sfx/cursor.ogg");
        self.items.iter().for_each(|item| item.change_opacity());

        let message = MessageListItem::new_choice(self.cx, message, choices);
        let height = message.get_height();
        self.items.push_front(message);
        if self.items.len() > 9 {
            let _ = self.items.pop_back();
        }

        for _ in 0..30 {
            for item in self.items.iter_mut() {
                item.set_pos(item.get_pos() + height / 30.0);
            }
            next_frame().await;
        }
    }

    async fn wait_choice(&self) -> anyhow::Result<usize> {
        self.items[0]
            .choice()
            .await
            .ok_or(anyhow::Error::msg("Last message is not choice type"))
    }
}

struct ConfirmGetSkillWindow<'a> {
    cx: &'a AnimationEngineContext,
}
impl<'a> ConfirmGetSkillWindow<'a> {
    fn new(cx: &'a AnimationEngineContext /*, skill: Skill */) -> Self {
        Self { cx }
    }

    async fn confirm(&self) -> bool {
        true
    }
}
impl<'a> Drop for ConfirmGetSkillWindow<'a> {
    fn drop(&mut self) {
        //
    }
}

struct ConfirmOverrideSkillWindow<'a> {
    cx: &'a AnimationEngineContext,
}
impl<'a> ConfirmOverrideSkillWindow<'a> {
    fn new(
        cx: &'a AnimationEngineContext, /*, current_skill: Skill, override_skill: Skill */
    ) -> Self {
        Self { cx }
    }

    async fn confirm(&self) -> bool {
        true
    }
}
impl<'a> Drop for ConfirmOverrideSkillWindow<'a> {
    fn drop(&mut self) {
        //
    }
}

struct SkillItemListWindow<'a> {
    cx: &'a AnimationEngineContext,
    cover: Entity,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
    part_4: Entity,
    message_text: Entity,
    header_text: Entity,
    description: Entity,
    list: Vec<Entity>,
}
impl<'a> SkillItemListWindow<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 100,
            ..Default::default()
        });
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-12.png".into(),
            x: 38.0,
            y: 200.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-13.png".into(),
            z: 115,
            a: 0.0,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-14.png".into(),
            x: 500.0,
            y: 148.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-15.png".into(),
            x: 125.0,
            y: 110.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let part_4 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-16.png".into(),
            x: 140.0,
            y: 20.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let message_text = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 170.0,
            y: 45.0,
            z: 120,
            a: 0.0,
            ..Default::default()
        });
        let header_text = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 190.0,
            y: 140.0,
            z: 120,
            a: 0.0,
            ..Default::default()
        });
        let description = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 610.0,
            y: 180.0,
            z: 120,
            a: 0.0,
            ..Default::default()
        });
        let list = (0..15)
            .map(|i| {
                cx.add_text(AddTextInfo {
                    font_size: 24.0,
                    x: 135.0 - 30.0 * i as f32 * 0.1763269807,
                    y: 230.0 + 30.0 * i as f32,
                    z: 120,
                    ..Default::default()
                })
            })
            .collect();
        Self {
            cx,
            cover,
            part_0,
            part_1,
            part_2,
            part_3,
            part_4,
            message_text,
            header_text,
            description,
            list,
        }
    }

    async fn add_skill(
        &self,
        player_data: &mut PlayerData,
        skill_id_list: &[usize],
        skill_list: &Vec<SkillData>,
    ) -> usize {
        let flag = ConfirmGetSkillWindow::new(self.cx).confirm().await;
        let flag = ConfirmOverrideSkillWindow::new(self.cx).confirm().await;
        0
    }

    fn set_cursor(&self, view_top_index: usize, cursor_index: usize, len: usize) {
        let i = cursor_index - view_top_index;
        self.cx
            .set_position(
                self.part_1,
                125.0 - 30.0 * i as f32 * 0.1763269807,
                220.0 + 30.0 * i as f32,
                115,
            )
            .unwrap();
        if len > 0 {
            self.cx.set_opacity(self.part_1, 1.0).unwrap();
        } else {
            self.cx.set_opacity(self.part_1, 0.0).unwrap();
        }
    }

    fn set_owned_skills(
        &self,
        view_top_index: usize,
        cursor_index: usize,
        player_state: &PlayerState,
        player_data: &PlayerData,
    ) {
        let mut skills = player_state.get_skills();
        skills.sort_by_key(|skill| {
            player_data
                .skills
                .iter()
                .find(|s| &s.id == skill)
                .unwrap()
                .skill_type
        });
        for index in view_top_index..(view_top_index + 15) {
            if let Some(skill) = skills.get(index) {
                let key = player_data
                    .skills
                    .iter()
                    .find(|s| &s.id == skill)
                    .unwrap()
                    .skill_name_with_level
                    .to_owned();
                self.cx.set_text_key(self.list[index], key).unwrap();
            } else {
                self.cx.set_text_key(self.list[index], "").unwrap();
            }
        }
        if let Some(skill) = skills.get(cursor_index) {
            let key = player_data
                .skills
                .iter()
                .find(|s| &s.id == skill)
                .unwrap()
                .skill_description
                .to_owned();
            self.cx.set_text_key(self.description, key).unwrap();
        } else {
            self.cx.set_text_key(self.description, "").unwrap();
        }
    }

    fn set_owned_items(
        &self,
        view_top_index: usize,
        cursor_index: usize,
        player_state: &PlayerState,
        item_data: &Vec<ItemData>,
    ) {
        let mut items = player_state.get_items();
        items.sort_by_key(|(item, _)| item_data.iter().find(|i| &i.id == item).unwrap().id);
        for index in view_top_index..(view_top_index + 15) {
            if let Some((item, count)) = items.get(index) {
                let key = item_data
                    .iter()
                    .find(|i| &i.id == item)
                    .unwrap()
                    .item_name_with_count
                    .to_owned();
                self.cx.set_text_key(self.list[index], key).unwrap();
                self.cx
                    .set_text_format_args(self.list[index], &[&count.to_string()])
                    .unwrap();
            } else {
                self.cx.set_text_key(self.list[index], "").unwrap();
            }
        }
        if let Some((item, _)) = items.get(cursor_index) {
            let key = item_data
                .iter()
                .find(|i| &i.id == item)
                .unwrap()
                .item_description
                .to_owned();
            self.cx.set_text_key(self.description, key).unwrap();
        } else {
            self.cx.set_text_key(self.description, "").unwrap();
        }
    }

    async fn show_skills_and_items(
        &self,
        player_state: &PlayerState,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
    ) {
        trace!("Open skills and items menu");

        self.cx
            .set_text_key(self.message_text, "explore-check-skills-and-items")
            .unwrap();
        self.cx
            .set_text_key(self.header_text, "explore-header-owned-skills")
            .unwrap();

        let mut view_top_index = [0, 0];
        let mut cursor_index = [0, 0];
        let mut page = 0;
        let len = [
            player_state.get_skills().len(),
            player_state.get_items().len(),
        ];

        self.set_cursor(view_top_index[page], cursor_index[page], len[page]);
        self.set_owned_skills(
            view_top_index[page],
            cursor_index[page],
            player_state,
            player_data,
        );

        try_join!(
            self.cx.play_animation(
                self.cover,
                "/animation/explore/skill-item-list-cover-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_0,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_2,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_3,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_4,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.message_text,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.header_text,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.description,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            futures::future::join_all(self.list.iter().map(|&entity| {
                self.cx
                    .play_animation(entity, "/animation/explore/skill-item-list-fade-in.yml")
            }))
            .map(|_| Ok(())),
        )
        .expect("animation not found");

        loop {
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len[page] > 0 {
                        cursor_index[page] = (cursor_index[page] - 1 + len[page]) % len[page];
                    }
                    if cursor_index[page] < view_top_index[page] {
                        view_top_index[page] = cursor_index[page]
                    }
                    if cursor_index[page] > view_top_index[page] + 15 {
                        view_top_index[page] = cursor_index[page] - 15
                    }
                },
                _ = input::wait_down(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len[page] > 0 {
                        cursor_index[page] = (cursor_index[page] + 1 + len[page]) % len[page];
                    }
                    if cursor_index[page] < view_top_index[page] {
                        view_top_index[page] = cursor_index[page]
                    }
                    if cursor_index[page] > view_top_index[page] + 15 {
                        view_top_index[page] = cursor_index[page] - 15
                    }
                },
                _ = input::wait_left(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    page = (page + 1) % 2;
                },
                _ = input::wait_right(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    page = (page + 1) % 2;
                },
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    break;
                },
            }
            if page == 0 {
                self.cx
                    .set_text_key(self.header_text, "explore-header-owned-skills")
                    .unwrap();
                self.set_owned_skills(
                    view_top_index[page],
                    cursor_index[page],
                    player_state,
                    player_data,
                );
            } else {
                self.cx
                    .set_text_key(self.header_text, "explore-header-owned-items")
                    .unwrap();
                self.set_owned_items(
                    view_top_index[page],
                    cursor_index[page],
                    player_state,
                    item_data,
                );
            }
            self.set_cursor(view_top_index[page], cursor_index[page], len[page]);
            delay(Duration::from_millis(150)).await;
        }

        self.cx.set_opacity(self.part_1, 0.0).unwrap();
        try_join!(
            self.cx.play_animation(
                self.cover,
                "/animation/explore/skill-item-list-cover-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_0,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_2,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_3,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_4,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.message_text,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.header_text,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.description,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            futures::future::join_all(self.list.iter().map(|&entity| {
                self.cx
                    .play_animation(entity, "/animation/explore/skill-item-list-fade-out.yml")
            }))
            .map(|_| Ok(())),
        )
        .expect("animation not found");
    }
}
impl<'a> Drop for SkillItemListWindow<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
        self.cx.delete_entity(self.part_4);
        self.cx.delete_entity(self.message_text);
        self.cx.delete_entity(self.header_text);
        self.cx.delete_entity(self.description);
        for &item in self.list.iter() {
            self.cx.delete_entity(item);
        }
    }
}

struct CurrentDepth<'a> {
    cx: &'a AnimationEngineContext,
    text: Entity,
    count_text: Entity,
    depth: u32,
    max_depth: u32,
}
impl<'a> CurrentDepth<'a> {
    fn new(cx: &'a AnimationEngineContext, max_depth: u32) -> Self {
        let text = cx.add_text(AddTextInfo {
            key: "explore-depth".into(),
            font_size: 48.0,
            x: 40.0,
            y: 40.0,
            z: 20,
            ..Default::default()
        });
        let count_text = cx.add_text(AddTextInfo {
            key: "explore-current-depth".into(),
            font_size: 36.0,
            format_args: vec!["  1".into(), format!("{}", max_depth)],
            x: 70.0,
            y: 90.0,
            z: 20,
            ..Default::default()
        });
        Self {
            cx,
            text,
            count_text,
            depth: 1,
            max_depth,
        }
    }

    fn increment(&mut self) {
        self.depth += 1;
        self.cx
            .set_text_format_args(
                self.count_text,
                &[&format!("{:3}", self.depth), &format!("{}", self.max_depth)],
            )
            .unwrap();
    }
}
impl<'a> Drop for CurrentDepth<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.text);
        self.cx.delete_entity(self.count_text);
    }
}

struct Background<'a> {
    cx: &'a AnimationEngineContext,
    bg: Entity,
    morning_cover: Entity,
    night_cover: Entity,
}
impl<'a> Background<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        let bg = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-bg.png".into(),
            x: 0.0,
            z: 0,
            ..Default::default()
        });
        let morning_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 233.0 / 255.0,
            b: 1.0,
            a: 0.25,
            z: 5,
            ..Default::default()
        });
        let night_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 28.0 / 255.0,
            b: 193.0 / 255.0,
            a: 0.0,
            z: 5,
            ..Default::default()
        });
        Self {
            cx,
            bg,
            morning_cover,
            night_cover,
        }
    }

    async fn change_to_afternoon(&self) {
        self.cx
            .play_animation(
                self.morning_cover,
                "/animation/explore/morning-cover-out.yml",
            )
            .await
            .expect("animation not found");
    }

    async fn change_to_night(&self) {
        self.cx
            .play_animation(self.night_cover, "/animation/explore/night-cover-in.yml")
            .await
            .expect("animation not found");
    }
}
impl<'a> Drop for Background<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.morning_cover);
        self.cx.delete_entity(self.night_cover);
    }
}

struct WindowFrame<'a> {
    cx: &'a AnimationEngineContext,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
}
impl<'a> WindowFrame<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-0.png".into(),
            z: 10,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-1.png".into(),
            z: 10,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-2.png".into(),
            x: 1280.0 - 148.0,
            z: 10,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-3.png".into(),
            x: 1280.0 - 105.0,
            y: 720.0 - 592.0,
            z: 10,
            ..Default::default()
        });
        Self {
            cx,
            part_0,
            part_1,
            part_2,
            part_3,
        }
    }
}
impl<'a> Drop for WindowFrame<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
    }
}

struct Cover<'a> {
    cx: &'a AnimationEngineContext,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
}
impl<'a> Cover<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-24.png".into(),
            y: 720.0,
            z: 200,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-25.png".into(),
            y: 720.0,
            z: 199,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-26.png".into(),
            y: 720.0,
            z: 198,
            ..Default::default()
        });
        let part_3 = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 200,
            ..Default::default()
        });
        Self {
            cx,
            part_0,
            part_1,
            part_2,
            part_3,
        }
    }

    async fn fade_in(&self) {
        self.cx.set_opacity(self.part_0, 0.0).unwrap();
        self.cx.set_opacity(self.part_1, 0.0).unwrap();
        self.cx.set_opacity(self.part_2, 0.0).unwrap();
        self.cx
            .play_animation(self.part_3, "/animation/explore/cover-fade-in.yml")
            .await
            .expect("animation not found");
    }

    async fn fade_out(&self) {
        self.cx.set_opacity(self.part_0, 0.0).unwrap();
        self.cx.set_opacity(self.part_1, 0.0).unwrap();
        self.cx.set_opacity(self.part_2, 0.0).unwrap();
        self.cx
            .play_animation(self.part_3, "/animation/explore/cover-fade-out.yml")
            .await
            .expect("animation not found");
    }

    async fn start_battle(&self) {
        self.cx.set_opacity(self.part_0, 1.0).unwrap();
        self.cx.set_opacity(self.part_1, 1.0).unwrap();
        self.cx.set_opacity(self.part_2, 1.0).unwrap();
        try_join!(
            self.cx
                .play_animation(self.part_3, "/animation/explore/cover-battle-start.yml"),
            self.cx
                .play_animation(self.part_2, "/animation/explore/cover-battle-start-1.yml"),
            self.cx
                .play_animation(self.part_1, "/animation/explore/cover-battle-start-2.yml"),
            self.cx
                .play_animation(self.part_0, "/animation/explore/cover-battle-start-3.yml"),
        )
        .expect("animation not found");
    }
}
impl<'a> Drop for Cover<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
    }
}

pub struct ExploreScene<'a> {
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
    fn new(cx: &'a AnimationEngineContext, player_index: usize, max_depth: u32) -> Self {
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

    async fn menu_and_options(&self) {
        loop {
            // メニューとオプションの待受を無限ループでする
            next_frame().await;
        }
    }

    async fn wait_move_forward(&self) {
        select! {
            _ = input::wait_select_button(self.cx).fuse() => (),
            _ = self.menu_and_options().fuse() => unreachable!(),
        }
    }

    async fn start(&mut self, global_data: &mut game::GlobalData) -> anyhow::Result<()> {
        let mut player_state = PlayerState::new();

        let player_data = &global_data.game_data().player_data()[self.player_index];
        let item_data = global_data.game_data().item_data();

        self.cx.play_bgm("field-0");
        self.cover.fade_in().await;

        let messages = [
            "message1", "message2", "message3", "message4", "message5", "message6",
        ];
        for message in messages {
            self.message_list.add_normal_message(message).await;
            self.wait_move_forward().await;
            self.current_depth.increment();
        }

        self.message_list.add_normal_message("message").await;
        select! {
            _ = self.skill_item_list_window
                .show_skills_and_items(&player_state, player_data, item_data)
                .fuse() => (),
            _ = self.menu_and_options().fuse() => unreachable!(),
        }

        player_state.add_item(0);
        player_state.add_item(0);
        player_state.add_item(0);
        player_state.add_item(0);
        player_state.add_item(1);
        player_state.add_item(2);
        player_state.add_item(3);
        player_state.add_item(4);
        player_state.add_skill(0, &player_data.skills);
        player_state.add_skill(1, &player_data.skills);
        player_state.add_skill(2, &player_data.skills);
        player_state.add_skill(3, &player_data.skills);

        self.message_list.add_normal_message("message").await;
        select! {
            _ = self.skill_item_list_window
                .show_skills_and_items(&player_state, player_data, item_data)
                .fuse() => (),
            _ = self.menu_and_options().fuse() => unreachable!(),
        }

        self.message_list
            .add_choice_message(
                "message-choice",
                &[
                    (2, "choice1"),
                    (1, "choice2"),
                    (3, "choice3"),
                    (1, "choice4"),
                ],
            )
            .await;
        select! {
            index = self.message_list.wait_choice().fuse()
                => info!("Choice: {}", index?),
            _ = self.menu_and_options().fuse() => (),
        }

        self.background.change_to_afternoon().await;
        let messages = [
            "message1", "message2", "message3", "message4", "message5", "message6",
        ];
        for message in messages {
            self.message_list.add_normal_message(message).await;
            self.wait_move_forward().await;
            self.current_depth.increment();
        }

        self.cx.play_bgm("battle-0");
        self.cover.start_battle().await;
        input::wait_select_button(self.cx).await;
        self.cx.stop_bgm();
        self.cover.fade_in().await;

        self.background.change_to_night().await;
        self.cx.play_bgm("field-1");
        let messages = [
            "message1", "message2", "message3", "message4", "message5", "message6",
        ];
        for message in messages {
            self.message_list.add_normal_message(message).await;
            self.wait_move_forward().await;
            self.current_depth.increment();
        }

        self.cover.fade_out().await;
        Ok(())
    }
}

pub async fn explore(
    cx: &AnimationEngineContext,
    global_data: &mut game::GlobalData,
    player_index: usize,
) {
    info!("Enter Explore Scene!");
    ExploreScene::new(cx, player_index, 150)
        .start(global_data)
        .await
        .expect("Failed to play explore scene.")
}
