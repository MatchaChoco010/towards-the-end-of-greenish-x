use animation_engine::executor::*;
use animation_engine::*;
use futures::future::try_join_all;
use futures::{select, FutureExt};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use crate::game_data::*;
use crate::input;

#[derive(Clone)]
pub(super) struct ItemWindowItem {
    pub name_key: String,
    pub active: bool,
}

pub(super) struct ItemsWindow<'a> {
    cx: &'a AnimationEngineContext,
    cover: Entity,
    part_9: Entity,
    part_10: Entity,
    part_11: Entity,
    description: Entity,
    items: Vec<ItemWindowItem>,
    item_name_entities: Vec<Entity>,
}
impl<'a> ItemsWindow<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 500,
            ..Default::default()
        });
        let part_9 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-9.png".into(),
            x: 60.0,
            y: 128.0,
            z: 505,
            a: 0.0,
            ..Default::default()
        });
        let part_10 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-10.png".into(),
            z: 507,
            a: 0.0,
            ..Default::default()
        });
        let part_11 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-11.png".into(),
            x: 512.0,
            y: 68.5,
            z: 505,
            a: 0.0,
            ..Default::default()
        });
        let description = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 575.0,
            y: 150.0,
            z: 510,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let items = vec![];
        let mut item_name_entities = vec![];
        for i in 0..11 {
            let item_name = cx.add_text(AddTextInfo {
                font_size: 24.0,
                x: 120.0 - 44.0 * 0.0872665 * i as f32,
                y: 194.0 + 44.0 * i as f32,
                z: 510,
                a: 0.0,
                rotation: -0.0872665,
                ..Default::default()
            });
            item_name_entities.push(item_name);
        }
        Self {
            cx,
            cover,
            part_9,
            part_10,
            part_11,
            description,
            items,
            item_name_entities,
        }
    }

    fn set_cursor(&mut self, description: impl ToString, top_index: usize, cursor_index: usize) {
        let cursor_pos = cursor_index - top_index;
        let x = 107.0 - 0.0872665 * 44.0 * cursor_pos as f32;
        let y = 152.0 + 44.0 * cursor_pos as f32;
        self.cx.set_position(self.part_10, x, y, 507).unwrap();
        for &entity in &self.item_name_entities {
            self.cx.set_text_key(entity, "").unwrap();
        }
        for i in 0..(self.items.len().min(11)) {
            self.cx
                .set_text_key(
                    self.item_name_entities[i],
                    self.items[top_index + i].name_key.to_owned(),
                )
                .unwrap();
            if self.items[top_index + i].active {
                self.cx
                    .set_opacity(self.item_name_entities[i], 1.0)
                    .unwrap();
            } else {
                if top_index + i == cursor_index {
                    self.cx
                        .set_opacity(self.item_name_entities[i], 0.6)
                        .unwrap();
                } else {
                    self.cx
                        .set_opacity(self.item_name_entities[i], 0.1)
                        .unwrap();
                }
            }
        }
        self.cx
            .set_text_key(self.description, description.to_string())
            .unwrap();
    }

    fn set_items(&mut self, items: Vec<ItemWindowItem>) {
        self.items = items;
    }

    async fn show(&self) {
        let mut anims: Vec<Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>> = vec![
            Box::pin(
                self.cx
                    .play_animation(self.cover, "/animation/battle/window-cover-fade-in.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_9, "/animation/battle/window-item-fade-in.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_10, "/animation/battle/window-item-fade-in.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_11, "/animation/battle/window-item-fade-in.yml"),
            ),
            Box::pin(self.cx.play_animation(
                self.description,
                "/animation/battle/window-item-fade-in.yml",
            )),
        ];
        for &entity in &self.item_name_entities {
            anims.push(Box::pin(self.cx.play_animation(
                entity,
                "/animation/battle/window-item-nonactive-fade-in.yml",
            )));
        }
        try_join_all(anims).await.expect("animation not found");
    }

    async fn hide(&self) {
        let mut anims: Vec<Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>> = vec![
            Box::pin(
                self.cx
                    .play_animation(self.cover, "/animation/battle/window-cover-fade-out.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_9, "/animation/battle/window-item-fade-out.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_10, "/animation/battle/window-item-fade-out.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_11, "/animation/battle/window-item-fade-out.yml"),
            ),
            Box::pin(self.cx.play_animation(
                self.description,
                "/animation/battle/window-item-fade-out.yml",
            )),
        ];
        for &entity in &self.item_name_entities {
            anims.push(Box::pin(self.cx.play_animation(
                entity,
                "/animation/battle/window-item-nonactive-fade-out.yml",
            )));
        }
        try_join_all(anims).await.expect("animation not found");
    }

    pub(super) async fn select_item<'b>(
        &mut self,
        items: &[ItemWindowItem],
        item_data: &[&'b ItemData],
    ) -> Option<&'b ItemData> {
        self.set_items(items.to_vec());

        let len = items.len();
        let mut view_top_index = 0;
        let mut cursor_index = 0;
        self.set_cursor(
            &item_data[cursor_index].item_description,
            view_top_index,
            cursor_index,
        );

        self.show().await;
        self.set_cursor(
            &item_data[cursor_index].item_description,
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
                    if items[cursor_index].active {
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
            self.set_cursor(
                &item_data[cursor_index].item_description,
                view_top_index,
                cursor_index,
            );
            delay(Duration::from_millis(150)).await;
        };
        self.hide().await;

        if canceled {
            None
        } else {
            Some(item_data[cursor_index])
        }
    }
}
impl<'a> Drop for ItemsWindow<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.part_9);
        self.cx.delete_entity(self.part_10);
        self.cx.delete_entity(self.part_11);
        self.cx.delete_entity(self.description);
        for entity in self.item_name_entities.drain(0..) {
            self.cx.delete_entity(entity);
        }
    }
}
