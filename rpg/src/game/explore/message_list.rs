use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, FutureExt};
use log::trace;
use std::collections::VecDeque;
use std::time::Duration;

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

    fn new_normal_blue(cx: &'a AnimationEngineContext, message: impl ToString) -> Self {
        trace!("NormalBlueMessage: {}", message.to_string());
        let window = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-11.png".into(),
            z: 40,
            ..Default::default()
        });
        let text = cx.add_text(AddTextInfo {
            key: message.to_string(),
            font_size: 24.0,
            z: 50,
            r: 0.3,
            g: 0.6,
            b: 1.0,
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
                z: 45,
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
                        45,
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

pub(super) struct MessageList<'a> {
    cx: &'a AnimationEngineContext,
    items: VecDeque<MessageListItem<'a>>,
}
impl<'a> MessageList<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let items = VecDeque::new();
        Self { cx, items }
    }

    pub(super) async fn add_normal_message(&mut self, message: impl ToString) {
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

    pub(super) async fn add_normal_blue_message(&mut self, message: impl ToString) {
        self.cx.play_sfx("/audio/sfx/cursor.ogg");
        self.items.iter().for_each(|item| item.change_opacity());

        let message = MessageListItem::new_normal_blue(self.cx, message);
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

    pub(super) async fn add_choice_message(
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

    pub(super) async fn wait_choice(&self) -> anyhow::Result<usize> {
        self.items[0]
            .choice()
            .await
            .ok_or(anyhow::Error::msg("Last message is not choice type"))
    }
}
