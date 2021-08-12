// use animation_engine::executor::*;
use animation_engine::*;
use async_recursion::async_recursion;
use futures::{select, try_join, FutureExt};
use log::{info, trace};
use rand::distributions::*;
use rand::prelude::*;

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
    mini_cursor_bg: Entity,
    mini_cursor_top: Entity,
    text_prologue: Entity,
    text_next: Entity,
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
            z: 290,
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
            z: 260,
            a: 0.0,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-3.png".into(),
            z: 255,
            a: 0.0,
            ..Default::default()
        });
        let part_4 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-4.png".into(),
            z: 250,
            a: 0.0,
            ..Default::default()
        });
        let part_5 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-5.png".into(),
            z: 255,
            a: 0.0,
            ..Default::default()
        });
        let part_6 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-6.png".into(),
            z: 250,
            a: 0.0,
            ..Default::default()
        });
        let part_7 = cx.add_image(AddImageInfo {
            name: "/image/ui/opening-part-7.png".into(),
            z: 220,
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
        let mini_cursor_bg = cx.add_image(AddImageInfo {
            name: "/image/ui/mini-cursor-bg.png".into(),
            z: 270,
            a: 0.0,
            ..Default::default()
        });
        let mini_cursor_top = cx.add_image(AddImageInfo {
            name: "/image/ui/mini-cursor-top.png".into(),
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
            mini_cursor_bg,
            mini_cursor_top,
            text_prologue,
            text_next,
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

    async fn prologue_index(
        &self,
        index: Option<usize>,
        opening_data: &OpeningData,
        rng: &mut ThreadRng,
    ) {
        match opening_data {
            OpeningData::Data { .. } => unreachable!(),
            OpeningData::Sequence { .. } => self.prologue(opening_data, rng).await,
            OpeningData::Random { data } => self.prologue(&data[index.unwrap()].1, rng).await,
        }
    }

    async fn start(&self, global_data: &mut game::GlobalData) -> PlayerIndex {
        self.cx.play_bgm("opening");
        self.enter_animation().await;

        let game_data = global_data.game_data();

        let index = match game_data.opening_data() {
            OpeningData::Data { .. } => unreachable!(),
            OpeningData::Sequence { .. } => None,
            OpeningData::Random { data } => {
                let dist = WeightedIndex::new(data.iter().map(|(w, _)| w)).unwrap();
                let index = dist.sample(&mut *global_data.rng());
                Some(index)
            }
        };

        self.prologue_index(index, game_data.opening_data(), &mut global_data.rng())
            .await;

        input::wait_select_button(self.cx).await;
        PlayerIndex(0)
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
        self.cx.delete_entity(self.mini_cursor_bg);
        self.cx.delete_entity(self.mini_cursor_top);
        self.cx.delete_entity(self.text_prologue);
        self.cx.delete_entity(self.text_next);
    }
}

pub async fn opening(
    cx: &AnimationEngineContext,
    global_data: &mut game::GlobalData,
) -> PlayerIndex {
    info!("Enter Opening Scene!");
    OpeningScene::new(cx).start(global_data).await
}
