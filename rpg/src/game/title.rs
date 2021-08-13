use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, try_join, FutureExt};
use log::{info, trace};
use std::time::Duration;

use crate::game::game;
use crate::game::options;
use crate::game::user_guide;
use crate::input;

pub enum TitleResult {
    StartGame,
    Exit,
}

struct TitleScene<'a> {
    cx: &'a AnimationEngineContext,
    bg: Entity,
    logo: Entity,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
    part_4: Entity,
    part_5: Entity,
    part_6: Entity,
    part_7: Entity,
    part_8: Entity,
    part_9: Entity,
    part_10: Entity,
    part_11: Entity,
    cover: Entity,
    text_game_start: Entity,
    text_monster_book: Entity,
    text_user_guide: Entity,
    text_options: Entity,
    text_exit: Entity,
}
impl<'a> TitleScene<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        trace!("new title scene.");

        let bg = cx.add_image(AddImageInfo {
            name: "/image/ui/title-bg.png".into(),
            z: 300,
            ..Default::default()
        });
        let logo = cx.add_image(AddImageInfo {
            name: "/image/ui/title-logo.png".into(),
            z: 380,
            ..Default::default()
        });
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-0.png".into(),
            z: 330,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-1.png".into(),
            z: 335,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-2.png".into(),
            z: 320,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-3.png".into(),
            z: 325,
            ..Default::default()
        });
        let part_4 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-4.png".into(),
            z: 310,
            ..Default::default()
        });
        let part_5 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-5.png".into(),
            z: 315,
            ..Default::default()
        });
        let part_6 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-6.png".into(),
            z: 340,
            ..Default::default()
        });
        let part_7 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-7.png".into(),
            z: 360,
            ..Default::default()
        });
        let part_8 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-8.png".into(),
            z: 370,
            ..Default::default()
        });
        let part_9 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-9.png".into(),
            z: 370,
            ..Default::default()
        });
        let part_10 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-10.png".into(),
            z: 370,
            ..Default::default()
        });
        let part_11 = cx.add_image(AddImageInfo {
            name: "/image/ui/title-part-11.png".into(),
            z: 370,
            ..Default::default()
        });
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 390,
            ..Default::default()
        });
        let text_game_start = cx.add_text(AddTextInfo {
            key: "game-start".into(),
            font_size: 36.0,
            x: 160.0 + 8.816349 * 4.0,
            y: 375.0,
            z: 350,
            r: 212.0 / 255.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let text_monster_book = cx.add_text(AddTextInfo {
            key: "monster-book".into(),
            font_size: 36.0,
            x: 160.0 + 8.816349 * 3.0,
            y: 425.0,
            z: 350,
            r: 212.0 / 255.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let text_user_guide = cx.add_text(AddTextInfo {
            key: "user-guide".into(),
            font_size: 36.0,
            x: 160.0 + 8.816349 * 2.0,
            y: 475.0,
            z: 350,
            r: 212.0 / 255.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let text_options = cx.add_text(AddTextInfo {
            key: "options".into(),
            font_size: 36.0,
            x: 160.0 + 8.816349,
            y: 525.0,
            z: 350,
            r: 212.0 / 255.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        let text_exit = cx.add_text(AddTextInfo {
            key: "exit".into(),
            font_size: 36.0,
            x: 160.0,
            y: 575.0,
            z: 350,
            r: 212.0 / 255.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
            ..Default::default()
        });
        Self {
            cx,
            bg,
            logo,
            part_0,
            part_1,
            part_2,
            part_3,
            part_4,
            part_5,
            part_6,
            part_7,
            part_8,
            part_9,
            part_10,
            part_11,
            cover,
            text_game_start,
            text_monster_book,
            text_user_guide,
            text_options,
            text_exit,
        }
    }

    async fn bg_loop_animation(&self) {
        loop {
            self.cx
                .play_animation(self.bg, "/animation/title/bg-loop.yml")
                .await
                .expect("animation not found");
        }
    }

    async fn enter_animation(&self) {
        try_join!(
            self.cx
                .play_animation(self.logo, "/animation/title/logo-enter.yml"),
            self.cx
                .play_animation(self.part_0, "/animation/title/part-0-enter.yml"),
            self.cx
                .play_animation(self.part_1, "/animation/title/part-1-enter.yml"),
            self.cx
                .play_animation(self.part_2, "/animation/title/part-2-enter.yml"),
            self.cx
                .play_animation(self.part_3, "/animation/title/part-3-enter.yml"),
            self.cx
                .play_animation(self.part_4, "/animation/title/part-4-enter.yml"),
            self.cx
                .play_animation(self.part_5, "/animation/title/part-5-enter.yml"),
            self.cx
                .play_animation(self.part_6, "/animation/title/part-6-enter.yml"),
            self.cx
                .play_animation(self.part_7, "/animation/title/part-7-enter.yml"),
            self.cx
                .play_animation(self.part_8, "/animation/title/part-8-enter.yml"),
            self.cx
                .play_animation(self.part_9, "/animation/title/part-9-enter.yml"),
            self.cx
                .play_animation(self.part_10, "/animation/title/part-10-enter.yml"),
            self.cx
                .play_animation(self.part_11, "/animation/title/part-11-enter.yml"),
            self.cx.play_animation(
                self.text_game_start,
                "/animation/title/menu-selected-enter.yml"
            ),
            self.cx
                .play_animation(self.text_monster_book, "/animation/title/menu-enter.yml"),
            self.cx
                .play_animation(self.text_user_guide, "/animation/title/menu-enter.yml"),
            self.cx
                .play_animation(self.text_options, "/animation/title/menu-enter.yml"),
            self.cx
                .play_animation(self.text_exit, "/animation/title/menu-enter.yml"),
        )
        .expect("animation not found");
    }

    async fn fade_animation(&self) {
        self.cx
            .play_animation(self.cover, "/animation/title/cover-fade.yml")
            .await
            .expect("animation not found");
    }

    async fn main(&self, global_data: &mut game::GlobalData) -> TitleResult {
        self.cx.play_bgm("title");
        self.cx.play_sfx("/audio/sfx/menu.ogg");
        self.enter_animation().await;

        let mut index = 0;
        loop {
            match index {
                0 => {
                    self.cx
                        .set_position(self.part_6, 173.0, 370.0, 340)
                        .unwrap();
                    self.cx
                        .set_position(self.part_7, 173.0, 370.0, 360)
                        .unwrap();
                    self.cx.set_opacity(self.text_game_start, 1.0).unwrap();
                    self.cx.set_opacity(self.text_monster_book, 0.2).unwrap();
                    self.cx.set_opacity(self.text_user_guide, 0.2).unwrap();
                    self.cx.set_opacity(self.text_options, 0.2).unwrap();
                    self.cx.set_opacity(self.text_exit, 0.2).unwrap();
                }
                1 => {
                    self.cx
                        .set_position(self.part_6, 173.0 - 8.816349, 420.0, 340)
                        .unwrap();
                    self.cx
                        .set_position(self.part_7, 173.0 - 8.816349, 420.0, 360)
                        .unwrap();
                    self.cx.set_opacity(self.text_game_start, 0.2).unwrap();
                    self.cx.set_opacity(self.text_monster_book, 1.5).unwrap();
                    self.cx.set_opacity(self.text_user_guide, 0.2).unwrap();
                    self.cx.set_opacity(self.text_options, 0.2).unwrap();
                    self.cx.set_opacity(self.text_exit, 0.2).unwrap();
                }
                2 => {
                    self.cx
                        .set_position(self.part_6, 173.0 - 8.816349 * 2.0, 470.0, 340)
                        .unwrap();
                    self.cx
                        .set_position(self.part_7, 173.0 - 8.816349 * 2.0, 470.0, 360)
                        .unwrap();
                    self.cx.set_opacity(self.text_game_start, 0.2).unwrap();
                    self.cx.set_opacity(self.text_monster_book, 0.2).unwrap();
                    self.cx.set_opacity(self.text_user_guide, 1.0).unwrap();
                    self.cx.set_opacity(self.text_options, 0.2).unwrap();
                    self.cx.set_opacity(self.text_exit, 0.2).unwrap();
                }
                3 => {
                    self.cx
                        .set_position(self.part_6, 173.0 - 8.816349 * 3.0, 520.0, 340)
                        .unwrap();
                    self.cx
                        .set_position(self.part_7, 173.0 - 8.816349 * 3.0, 520.0, 360)
                        .unwrap();
                    self.cx.set_opacity(self.text_game_start, 0.2).unwrap();
                    self.cx.set_opacity(self.text_monster_book, 0.2).unwrap();
                    self.cx.set_opacity(self.text_user_guide, 0.2).unwrap();
                    self.cx.set_opacity(self.text_options, 1.0).unwrap();
                    self.cx.set_opacity(self.text_exit, 0.2).unwrap();
                }
                4 => {
                    self.cx
                        .set_position(self.part_6, 173.0 - 8.816349 * 4.0, 570.0, 340)
                        .unwrap();
                    self.cx
                        .set_position(self.part_7, 173.0 - 8.816349 * 4.0, 570.0, 360)
                        .unwrap();
                    self.cx.set_opacity(self.text_game_start, 0.2).unwrap();
                    self.cx.set_opacity(self.text_monster_book, 0.2).unwrap();
                    self.cx.set_opacity(self.text_user_guide, 0.2).unwrap();
                    self.cx.set_opacity(self.text_options, 0.2).unwrap();
                    self.cx.set_opacity(self.text_exit, 1.0).unwrap();
                }
                _ => unreachable!(),
            }
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    index = (index - 1 + 5) % 5;
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    delay(Duration::from_millis(150)).await;
                }
                _ = input::wait_down(self.cx).fuse() => {
                    index = (index + 1 + 5) % 5;
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    delay(Duration::from_millis(150)).await;
                }
                _ = input::wait_select_button(self.cx).fuse() => {
                    match index {
                        0 => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                            self.fade_animation().await;
                            return TitleResult::StartGame;
                        },
                        1 => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                        },
                        2 => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                            user_guide::user_guide(self.cx).await
                        }
                        3 => {
                            self.cx.play_sfx("/audio/sfx/select.ogg");
                            options::options(self.cx, global_data).await;
                        }
                        4 => {
                            self.cx.play_sfx("/audio/sfx/cancel.ogg");
                            self.fade_animation().await;
                            return TitleResult::Exit;
                        },
                        _ => unreachable!(),
                    }
                }
                _ = input::wait_sub_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                    options::options(self.cx, global_data).await;
                }
            }
            next_frame().await;
        }
    }

    async fn start(&self, global_data: &mut game::GlobalData) -> TitleResult {
        select! {
            _ = self.bg_loop_animation().fuse() => TitleResult::Exit,
            result = self.main(global_data).fuse() => result,
        }
    }
}
impl<'a> Drop for TitleScene<'a> {
    fn drop(&mut self) {
        trace!("drop title scene.");

        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.logo);
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
        self.cx.delete_entity(self.part_4);
        self.cx.delete_entity(self.part_5);
        self.cx.delete_entity(self.part_6);
        self.cx.delete_entity(self.part_7);
        self.cx.delete_entity(self.part_8);
        self.cx.delete_entity(self.part_9);
        self.cx.delete_entity(self.part_10);
        self.cx.delete_entity(self.part_11);
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.text_game_start);
        self.cx.delete_entity(self.text_monster_book);
        self.cx.delete_entity(self.text_user_guide);
        self.cx.delete_entity(self.text_options);
        self.cx.delete_entity(self.text_exit);
    }
}

pub async fn title(cx: &AnimationEngineContext, global_data: &mut game::GlobalData) -> TitleResult {
    info!("Enter Title Scene!");
    TitleScene::new(cx).start(global_data).await
}
