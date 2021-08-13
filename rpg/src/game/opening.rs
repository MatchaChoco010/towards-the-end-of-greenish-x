use animation_engine::executor::next_frame;
use animation_engine::executor::*;
use animation_engine::*;
use async_recursion::async_recursion;
use futures::{select, try_join, FutureExt};
use log::{info, trace};
use rand::distributions::*;
use rand::prelude::*;
use std::time::Duration;

use crate::game::game;
use crate::game_data::*;
use crate::input;

pub struct PlayerIndex(pub usize);

struct OpeningScene<'a> {
    cx: &'a AnimationEngineContext,
    bg: Entity,
    cover: Entity,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
    part_4: Entity,
    part_5: Entity,
    part_6: Entity,
    part_7: Entity,
    cursor_bg: Entity,
    cursor_top: Entity,
    text_prologue: Entity,
    text_next: Entity,
    middle_cover: Entity,
    player_image: Entity,
    player_shadow_image: Entity,
    introduction_text: Entity,
    legendary_name_text: Entity,
    legendary_name: Entity,
    legendary_name_shadow: Entity,
    mini_cursor_bg: Entity,
    mini_cursor_top: Entity,
    confirm_message: Entity,
    yes: Entity,
    no: Entity,
}
impl<'a> OpeningScene<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        trace!("new opening scene.");

        let bg = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-bg.png".into(),
            x: 400.0,
            z: 200,
            ..Default::default()
        });
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
            z: 400,
            ..Default::default()
        });
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-0.png".into(),
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-1.png".into(),
            z: 205,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-2.png".into(),
            x: 200.0,
            y: 400.0,
            z: 380,
            a: 0.0,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-3.png".into(),
            x: 1080.0,
            y: 300.0,
            z: 355,
            a: 0.0,
            ..Default::default()
        });
        let part_4 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-4.png".into(),
            x: 1120.0,
            y: 300.0,
            z: 350,
            a: 0.0,
            ..Default::default()
        });
        let part_5 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-5.png".into(),
            x: 50.0,
            y: 300.0,
            z: 355,
            a: 0.0,
            ..Default::default()
        });
        let part_6 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-6.png".into(),
            x: 10.0,
            y: 300.0,
            z: 350,
            a: 0.0,
            ..Default::default()
        });
        let part_7 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-7.png".into(),
            z: 310,
            a: 0.0,
            ..Default::default()
        });
        let cursor_bg = cx.add_image(AddImageInfo {
            name: "/image/ui/cursor-bg.png".into(),
            x: 100.0,
            y: 620.0,
            z: 270,
            a: 0.0,
            ..Default::default()
        });
        let cursor_top = cx.add_image(AddImageInfo {
            name: "/image/ui/cursor-top.png".into(),
            x: 100.0,
            y: 620.0,
            z: 280,
            a: 0.0,
            ..Default::default()
        });
        let text_prologue = cx.add_text(AddTextInfo {
            font_size: 26.0,
            x: 120.0,
            y: 60.0,
            z: 275,
            r: 212.0 / 255.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let text_next = cx.add_text(AddTextInfo {
            key: "opening-next".into(),
            font_size: 36.0,
            x: 150.0,
            y: 623.0,
            z: 275,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let middle_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.3,
            g: 0.3,
            b: 0.3,
            a: 0.0,
            z: 300,
            ..Default::default()
        });
        let player_image = cx.add_image(AddImageInfo {
            y: 0.0,
            z: 340,
            a: 0.0,
            ..Default::default()
        });
        let player_shadow_image = cx.add_image(AddImageInfo {
            z: 335,
            a: 0.0,
            ..Default::default()
        });
        let introduction_text = cx.add_text(AddTextInfo {
            font_size: 26.0,
            y: 60.0,
            z: 330,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            ..Default::default()
        });
        let legendary_name_text = cx.add_text(AddTextInfo {
            key: "opening-legendary-name-text".into(),
            font_size: 26.0,
            y: 400.0,
            z: 345,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            ..Default::default()
        });
        let legendary_name = cx.add_text(AddTextInfo {
            font_size: 56.0,
            y: 450.0,
            z: 345,
            r: 0.8,
            g: 0.8,
            b: 0.8,
            a: 0.0,
            ..Default::default()
        });
        let legendary_name_shadow = cx.add_text(AddTextInfo {
            font_size: 56.0,
            y: 453.0,
            z: 344,
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 0.0,
            ..Default::default()
        });
        let mini_cursor_bg = cx.add_image(AddImageInfo {
            name: "/image/ui/mini-cursor-bg.png".into(),
            z: 385,
            a: 0.0,
            ..Default::default()
        });
        let mini_cursor_top = cx.add_image(AddImageInfo {
            name: "/image/ui/mini-cursor-top.png".into(),
            z: 395,
            a: 0.0,
            ..Default::default()
        });
        let confirm_message = cx.add_text(AddTextInfo {
            font_size: 36.0,
            x: 400.0,
            y: 440.0,
            z: 390,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let yes = cx.add_text(AddTextInfo {
            key: "opening-yes".into(),
            font_size: 36.0,
            x: 440.0,
            y: 595.0,
            z: 390,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let no = cx.add_text(AddTextInfo {
            key: "opening-no".into(),
            font_size: 36.0,
            x: 680.0,
            y: 595.0,
            z: 390,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        Self {
            cx,
            bg,
            cover,
            part_0,
            part_1,
            part_2,
            part_3,
            part_4,
            part_5,
            part_6,
            part_7,
            cursor_bg,
            cursor_top,
            text_prologue,
            text_next,
            middle_cover,
            player_image,
            player_shadow_image,
            introduction_text,
            legendary_name_text,
            legendary_name,
            legendary_name_shadow,
            mini_cursor_bg,
            mini_cursor_top,
            confirm_message,
            yes,
            no,
        }
    }

    async fn enter_animation(&self) {
        trace!("start opening enter animation");

        try_join!(
            self.cx
                .play_animation(self.part_1, "/animation/opening/part-1-enter.yml"),
            self.cx
                .play_animation(self.cover, "/animation/opening/cover-enter.yml"),
        )
        .expect("animation not found");

        trace!("finish opening enter animation");
    }

    #[async_recursion(?Send)]
    async fn prologue(&self, opening_data: &OpeningData, rng: &mut ThreadRng) {
        match opening_data {
            OpeningData::Data { data } => {
                self.cx
                    .set_text_key(self.text_prologue, data)
                    .expect(&format!("Failed to get text: {}", data));

                self.cx
                    .play_animation(
                        self.text_prologue,
                        "/animation/opening/text-prologue-fade-in.yml",
                    )
                    .await
                    .expect("animation not found");

                self.cx.play_sfx("/audio/sfx/cursor.ogg");
                try_join!(
                    self.cx
                        .play_animation(self.text_next, "/animation/opening/next-fade-in.yml"),
                    self.cx
                        .play_animation(self.cursor_bg, "/animation/opening/next-fade-in.yml"),
                    self.cx
                        .play_animation(self.cursor_top, "/animation/opening/next-fade-in.yml")
                )
                .expect("animation not found");

                input::wait_select_button(self.cx).await;

                self.cx.play_sfx("/audio/sfx/select.ogg");
                try_join!(
                    self.cx.play_animation(
                        self.text_prologue,
                        "/animation/opening/text-prologue-fade-out.yml",
                    ),
                    self.cx
                        .play_animation(self.text_next, "/animation/opening/next-fade-out.yml"),
                    self.cx
                        .play_animation(self.cursor_bg, "/animation/opening/next-fade-out.yml"),
                    self.cx
                        .play_animation(self.cursor_top, "/animation/opening/next-fade-out.yml")
                )
                .expect("animation not found");
            }
            OpeningData::Sequence { data } => {
                for text in data.iter() {
                    self.prologue(text, rng).await;
                }
            }
            OpeningData::Random { data } => {
                let dist = WeightedIndex::new(data.iter().map(|(w, _)| w)).unwrap();
                let index = dist.sample(rng);
                let opening_data = &data[index].1;
                self.prologue(opening_data, rng).await;
            }
        }
    }

    async fn prologue_index(&self, index: usize, opening_data: &OpeningData, rng: &mut ThreadRng) {
        trace!("start opening prologue: {}", index);

        match opening_data {
            OpeningData::Random { data } => self.prologue(&data[index].1, rng).await,
            _ => unreachable!(),
        }

        trace!("finish opening prologue: {}", index);
    }

    async fn enter_player_select_animation(&self) {
        trace!("start opening enter player select animation");

        try_join!(
            self.cx.play_animation(
                self.middle_cover,
                "/animation/opening/middle-cover-enter.yml"
            ),
            self.cx
                .play_animation(self.part_7, "/animation/opening/part-7-enter.yml"),
        )
        .expect("animation not found");
        try_join!(
            self.cx
                .play_animation(self.part_3, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.part_4, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.part_5, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.part_6, "/animation/opening/content-enter.yml"),
        )
        .expect("animation not found");

        trace!("finish opening enter player select animation");
    }

    async fn close_player_select_animation(&self) {
        trace!("start opening close player select animation");

        try_join!(
            self.cx.play_animation(
                self.middle_cover,
                "/animation/opening/middle-cover-close.yml"
            ),
            self.cx
                .play_animation(self.part_7, "/animation/opening/part-7-close.yml"),
            self.cx
                .play_animation(self.part_3, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.part_4, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.part_5, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.part_6, "/animation/opening/content-close.yml"),
            self.cx.play_animation(
                self.introduction_text,
                "/animation/opening/content-close.yml"
            ),
            self.cx
                .play_animation(self.legendary_name, "/animation/opening/content-close.yml"),
            self.cx.play_animation(
                self.legendary_name_shadow,
                "/animation/opening/content-close.yml"
            ),
            self.cx.play_animation(
                self.legendary_name_text,
                "/animation/opening/content-close.yml"
            ),
            self.cx
                .play_animation(self.part_2, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.confirm_message, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.yes, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.no, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.mini_cursor_bg, "/animation/opening/content-close.yml"),
            self.cx
                .play_animation(self.mini_cursor_top, "/animation/opening/content-close.yml"),
        )
        .expect("animation not found");

        trace!("finish opening close player select animation");
    }

    async fn player_fade_in(&self, player_data: &PlayerData) {
        self.cx
            .set_image_name(self.player_image, &player_data.image)
            .expect("Player image not found");
        self.cx
            .set_image_name(self.player_shadow_image, &player_data.shadow_image)
            .expect("Player image not found");
        self.cx
            .set_text_key(self.legendary_name, &player_data.opening_legendary_name)
            .expect(&format!(
                "Failed to get text: {}",
                &player_data.opening_legendary_name
            ));
        self.cx
            .set_text_key(
                self.legendary_name_shadow,
                &player_data.opening_legendary_name,
            )
            .expect(&format!(
                "Failed to get text: {}",
                &player_data.opening_legendary_name
            ));
        self.cx
            .set_text_key(
                self.introduction_text,
                &player_data.opening_introduction_text,
            )
            .expect(&format!(
                "Failed to get text: {}",
                &player_data.opening_introduction_text
            ));

        try_join!(
            self.cx.play_animation(
                self.player_image,
                "/animation/opening/player-image-fade-in.yml"
            ),
            self.cx.play_animation(
                self.player_shadow_image,
                "/animation/opening/player-shadow-image-fade-in.yml"
            ),
            self.cx.play_animation(
                self.introduction_text,
                "/animation/opening/introduction-text-fade-in.yml"
            ),
            self.cx.play_animation(
                self.legendary_name,
                "/animation/opening/legendary-name-fade-in.yml"
            ),
            self.cx.play_animation(
                self.legendary_name_shadow,
                "/animation/opening/legendary-name-shadow-fade-in.yml"
            ),
            self.cx.play_animation(
                self.legendary_name_text,
                "/animation/opening/legendary-name-text-fade-in.yml"
            ),
        )
        .expect("animation not found");
    }

    async fn player_fade_out(&self) {
        try_join!(
            self.cx.play_animation(
                self.player_image,
                "/animation/opening/player-image-fade-out.yml"
            ),
            self.cx.play_animation(
                self.player_shadow_image,
                "/animation/opening/player-shadow-image-fade-out.yml"
            ),
            self.cx.play_animation(
                self.introduction_text,
                "/animation/opening/introduction-text-fade-out.yml"
            ),
            self.cx.play_animation(
                self.legendary_name,
                "/animation/opening/legendary-name-fade-out.yml"
            ),
            self.cx.play_animation(
                self.legendary_name_shadow,
                "/animation/opening/legendary-name-shadow-fade-out.yml"
            ),
            self.cx.play_animation(
                self.legendary_name_text,
                "/animation/opening/legendary-name-text-fade-out.yml"
            ),
        )
        .expect("animation not found");
    }

    async fn confirm_player_select(&self, player_index: usize) -> bool {
        self.cx
            .set_text_key(
                self.confirm_message,
                format!("opening-player-{}-confirm", player_index),
            )
            .expect(&format!(
                "Failed to get text: {}",
                format!("opening-player-{}-confirm", player_index)
            ));
        self.cx
            .set_position(self.mini_cursor_bg, 620.0, 580.0, 385)
            .unwrap();
        self.cx
            .set_position(self.mini_cursor_top, 620.0, 580.0, 395)
            .unwrap();

        try_join!(
            self.cx
                .play_animation(self.part_2, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.mini_cursor_bg, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.mini_cursor_top, "/animation/opening/content-enter.yml"),
        )
        .expect("animation not found");
        try_join!(
            self.cx
                .play_animation(self.confirm_message, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.yes, "/animation/opening/content-enter.yml"),
            self.cx
                .play_animation(self.no, "/animation/opening/content-enter.yml"),
        )
        .expect("animation not found");

        let mut confirm = false;
        loop {
            select! {
                _ = input::wait_left(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    confirm = !confirm;
                    let pos_x = if confirm { 380.0 } else { 620.0 };
                    self.cx
                        .set_position(self.mini_cursor_bg, pos_x, 580.0, 385)
                        .unwrap();
                    self.cx
                        .set_position(self.mini_cursor_top, pos_x, 580.0, 395)
                        .unwrap();
                    next_frame().await;
                    delay(Duration::from_millis(150)).await;
                }
                _ = input::wait_right(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    confirm = !confirm;
                    let pos_x = if confirm { 380.0 } else { 620.0 };
                    self.cx
                        .set_position(self.mini_cursor_bg, pos_x, 580.0, 385)
                        .unwrap();
                    self.cx
                        .set_position(self.mini_cursor_top, pos_x, 580.0, 395)
                        .unwrap();
                    next_frame().await;
                    delay(Duration::from_millis(150)).await;
                }
                _ = input::wait_select_button(self.cx).fuse() => {
                    if confirm {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        return true;
                    } else {
                        self.cx.play_sfx("/audio/sfx/cancel.ogg");
                        try_join!(
                            self.cx
                                .play_animation(self.part_2, "/animation/opening/content-close.yml"),
                            self.cx
                                .play_animation(self.mini_cursor_bg, "/animation/opening/content-close.yml"),
                            self.cx
                                .play_animation(self.mini_cursor_top, "/animation/opening/content-close.yml"),
                            self.cx
                                .play_animation(self.confirm_message, "/animation/opening/content-close.yml"),
                            self.cx
                                .play_animation(self.yes, "/animation/opening/content-close.yml"),
                            self.cx
                                .play_animation(self.no, "/animation/opening/content-close.yml"),
                        )
                        .expect("animation not found");
                        return false;
                    }
                }
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    try_join!(
                        self.cx
                            .play_animation(self.part_2, "/animation/opening/content-close.yml"),
                        self.cx
                            .play_animation(self.mini_cursor_bg, "/animation/opening/content-close.yml"),
                        self.cx
                            .play_animation(self.mini_cursor_top, "/animation/opening/content-close.yml"),
                        self.cx
                            .play_animation(self.confirm_message, "/animation/opening/content-close.yml"),
                        self.cx
                            .play_animation(self.yes, "/animation/opening/content-close.yml"),
                        self.cx
                            .play_animation(self.no, "/animation/opening/content-close.yml"),
                    )
                    .expect("animation not found");
                    return false;
                }
            }
        }
    }

    async fn player_select(&self, player_data: &Vec<PlayerData>) -> PlayerIndex {
        self.enter_player_select_animation().await;

        let len = player_data.len();
        let mut player_index = 0;
        self.player_fade_in(&player_data[player_index]).await;
        loop {
            select! {
                _ = input::wait_left(self.cx).fuse() => {
                    player_index = (player_index - 1 + len) % len;
                    self.cx.play_sfx("/audio/sfx/menu.ogg");
                    self.player_fade_out().await;
                    self.player_fade_in(&player_data[player_index]).await;
                }
                _ = input::wait_right(self.cx).fuse() => {
                    player_index = (player_index + 1 + len) % len;
                    self.cx.play_sfx("/audio/sfx/menu.ogg");
                    self.player_fade_out().await;
                    self.player_fade_in(&player_data[player_index]).await;
                }
                _ = input::wait_select_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                    if self.confirm_player_select(player_index).await {
                        break;
                    }
                    next_frame().await;
                }
            }
        }

        self.close_player_select_animation().await;
        PlayerIndex(player_index)
    }

    async fn enter_player_prologue_animation(&self) {
        trace!("start opening enter player prologue animation");

        try_join!(
            self.cx
                .play_animation(self.part_0, "/animation/opening/part-0-enter.yml"),
            self.cx.play_animation(
                self.player_image,
                "/animation/opening/player-image-enter-prologue.yml"
            ),
            self.cx.play_animation(
                self.player_shadow_image,
                "/animation/opening/player-shadow-image-enter-prologue.yml"
            ),
        )
        .expect("animation not found");

        trace!("finish opening enter player prologue animation");
    }

    async fn player_prologue(&self, index: usize, player_data: &PlayerData) {
        self.enter_player_prologue_animation().await;

        let messages = &player_data
            .prologue
            .iter()
            .find(|i| i.index == index)
            .unwrap()
            .messages;
        for PrologueMessage(text) in messages {
            self.cx
                .set_text_key(self.text_prologue, text)
                .expect(&format!("Failed to get text: {}", text));

            self.cx
                .play_animation(
                    self.text_prologue,
                    "/animation/opening/text-prologue-fade-in.yml",
                )
                .await
                .expect("animation not found");

            self.cx.play_sfx("/audio/sfx/cursor.ogg");
            try_join!(
                self.cx
                    .play_animation(self.text_next, "/animation/opening/next-fade-in.yml"),
                self.cx
                    .play_animation(self.cursor_bg, "/animation/opening/next-fade-in.yml"),
                self.cx
                    .play_animation(self.cursor_top, "/animation/opening/next-fade-in.yml")
            )
            .expect("animation not found");

            input::wait_select_button(self.cx).await;

            self.cx.play_sfx("/audio/sfx/select.ogg");
            try_join!(
                self.cx.play_animation(
                    self.text_prologue,
                    "/animation/opening/text-prologue-fade-out.yml",
                ),
                self.cx
                    .play_animation(self.text_next, "/animation/opening/next-fade-out.yml"),
                self.cx
                    .play_animation(self.cursor_bg, "/animation/opening/next-fade-out.yml"),
                self.cx
                    .play_animation(self.cursor_top, "/animation/opening/next-fade-out.yml")
            )
            .expect("animation not found");
        }
    }

    async fn close_animation(&self) {
        trace!("start opening close animation");

        self.cx
            .play_animation(self.cover, "/animation/opening/cover-close.yml")
            .await
            .expect("animation not founr");

        trace!("finish opening close animation");
    }

    async fn start(&self, global_data: &mut game::GlobalData) -> PlayerIndex {
        self.cx.play_bgm("opening");
        self.enter_animation().await;

        let game_data = global_data.game_data();
        let index = match game_data.opening_data() {
            OpeningData::Random { data } => {
                let dist = WeightedIndex::new(data.iter().map(|(w, _)| w)).unwrap();
                dist.sample(&mut *global_data.rng())
            }
            _ => unreachable!(),
        };
        self.prologue_index(index, game_data.opening_data(), &mut global_data.rng())
            .await;
        let player_index = self.player_select(game_data.player_data()).await;
        self.player_prologue(index, &game_data.player_data()[player_index.0])
            .await;

        self.close_animation().await;
        player_index
    }
}
impl<'a> Drop for OpeningScene<'a> {
    fn drop(&mut self) {
        trace!("drop opening scene.");

        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
        self.cx.delete_entity(self.part_4);
        self.cx.delete_entity(self.part_5);
        self.cx.delete_entity(self.part_6);
        self.cx.delete_entity(self.part_7);
        self.cx.delete_entity(self.cursor_bg);
        self.cx.delete_entity(self.cursor_top);
        self.cx.delete_entity(self.text_prologue);
        self.cx.delete_entity(self.text_next);
        self.cx.delete_entity(self.middle_cover);
        self.cx.delete_entity(self.player_image);
        self.cx.delete_entity(self.player_shadow_image);
        self.cx.delete_entity(self.introduction_text);
        self.cx.delete_entity(self.legendary_name_text);
        self.cx.delete_entity(self.legendary_name);
        self.cx.delete_entity(self.legendary_name_shadow);
        self.cx.delete_entity(self.mini_cursor_bg);
        self.cx.delete_entity(self.mini_cursor_top);
        self.cx.delete_entity(self.confirm_message);
        self.cx.delete_entity(self.yes);
        self.cx.delete_entity(self.no);
    }
}

pub async fn opening(
    cx: &AnimationEngineContext,
    global_data: &mut game::GlobalData,
) -> PlayerIndex {
    info!("Enter Opening Scene!");
    OpeningScene::new(cx).start(global_data).await
}
