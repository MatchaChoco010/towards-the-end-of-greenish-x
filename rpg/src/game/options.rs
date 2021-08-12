use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, try_join, FutureExt};
use log::{info, trace};

use crate::game::game;
use crate::input;
use crate::localization;

struct Options<'a> {
    cx: &'a AnimationEngineContext,
    bg_cover: Entity,
    bg: Entity,
    cursor: Entity,
    arrow_bgm: Entity,
    arrow_sfx: Entity,
    arrow_lang: Entity,
    text_title: Entity,
    text_bgm_title: Entity,
    text_bgm_value: Entity,
    text_sfx_title: Entity,
    text_sfx_value: Entity,
    text_lang_title: Entity,
    text_lang_value: Entity,
    text_close: Entity,
}
impl<'a> Options<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        let bg_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            z: 400,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.4,
            ..Default::default()
        });
        let bg = cx.add_image(AddImageInfo {
            name: "/image/ui/menu-bg.png".into(),
            x: -25.0,
            y: 0.0,
            z: 405,
            ..Default::default()
        });
        let cursor = cx.add_image(AddImageInfo {
            name: "/image/ui/options-part-1.png".into(),
            x: 200.0,
            y: 150.0,
            z: 405,
            ..Default::default()
        });
        let arrow_bgm = cx.add_image(AddImageInfo {
            name: "/image/ui/options-part-3.png".into(),
            x: 550.0,
            y: 170.0,
            z: 410,
            ..Default::default()
        });
        let arrow_sfx = cx.add_image(AddImageInfo {
            name: "/image/ui/options-part-3.png".into(),
            x: 550.0 - 0.17632698 * 130.0,
            y: 300.0,
            z: 410,
            ..Default::default()
        });
        let arrow_lang = cx.add_image(AddImageInfo {
            name: "/image/ui/options-part-2.png".into(),
            x: 550.0 - 0.17632698 * 130.0 * 2.0,
            y: 430.0,
            z: 410,
            ..Default::default()
        });
        let text_title = cx.add_text(AddTextInfo {
            key: "options-title".into(),
            font_size: 72.0,
            x: 550.0,
            y: 30.0,
            z: 410,
            ..Default::default()
        });
        let text_bgm_title = cx.add_text(AddTextInfo {
            key: "options-bgm".into(),
            font_size: 56.0,
            x: 250.0,
            y: 165.0,
            z: 410,
            ..Default::default()
        });
        let text_bgm_value = cx.add_text(AddTextInfo {
            key: "options-volume-70".into(),
            font_size: 56.0,
            x: 630.0,
            y: 165.0,
            z: 410,
            ..Default::default()
        });
        let text_sfx_title = cx.add_text(AddTextInfo {
            key: "options-se".into(),
            font_size: 56.0,
            x: 250.0 - 0.17632698 * 130.0,
            y: 295.0,
            z: 410,
            ..Default::default()
        });
        let text_sfx_value = cx.add_text(AddTextInfo {
            key: "options-volume-70".into(),
            font_size: 56.0,
            x: 630.0 - 0.17632698 * 130.0,
            y: 295.0,
            z: 410,
            ..Default::default()
        });
        let text_lang_title = cx.add_text(AddTextInfo {
            key: "options-language".into(),
            font_size: 56.0,
            x: 250.0 - 0.17632698 * 130.0 * 2.0,
            y: 425.0,
            z: 410,
            ..Default::default()
        });
        let text_lang_value = cx.add_text(AddTextInfo {
            key: "options-language-name".into(),
            font_size: 36.0,
            x: 555.0,
            y: 435.0,
            z: 410,
            ..Default::default()
        });
        let text_close = cx.add_text(AddTextInfo {
            key: "options-exit".into(),
            font_size: 56.0,
            x: 500.0,
            y: 555.0,
            z: 410,
            ..Default::default()
        });
        Self {
            cx,
            bg,
            bg_cover,
            cursor,
            arrow_bgm,
            arrow_sfx,
            arrow_lang,
            text_title,
            text_bgm_title,
            text_bgm_value,
            text_sfx_title,
            text_sfx_value,
            text_lang_title,
            text_lang_value,
            text_close,
        }
    }

    fn volume_to_key(volume: u8) -> String {
        match volume {
            0 => "options-volume-0".into(),
            1 => "options-volume-10".into(),
            2 => "options-volume-20".into(),
            3 => "options-volume-30".into(),
            4 => "options-volume-40".into(),
            5 => "options-volume-50".into(),
            6 => "options-volume-60".into(),
            7 => "options-volume-70".into(),
            8 => "options-volume-80".into(),
            9 => "options-volume-90".into(),
            10 => "options-volume-100".into(),
            11 => "options-volume-110".into(),
            12 => "options-volume-120".into(),
            13 => "options-volume-130".into(),
            14 => "options-volume-140".into(),
            15 => "options-volume-150".into(),
            _ => unreachable!(),
        }
    }

    async fn enter_animation(&self) {
        trace!("Start Options enter animation");

        try_join!(
            self.cx
                .play_animation(self.bg_cover, "/animation/options/cover-enter.yml"),
            self.cx
                .play_animation(self.bg, "/animation/options/bg-enter.yml"),
            self.cx
                .play_animation(self.cursor, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.arrow_bgm, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.arrow_sfx, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.arrow_lang, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_title, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_bgm_title, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_bgm_value, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_sfx_title, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_sfx_value, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_lang_title, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_lang_value, "/animation/options/content-enter.yml"),
            self.cx
                .play_animation(self.text_close, "/animation/options/content-enter.yml"),
        )
        .expect("animation not found");

        trace!("Finish Options enter animation");
    }

    async fn close_animation(&self) {
        trace!("Start Options close animation");

        try_join!(
            self.cx
                .play_animation(self.bg_cover, "/animation/options/cover-close.yml"),
            self.cx
                .play_animation(self.bg, "/animation/options/bg-close.yml"),
            self.cx
                .play_animation(self.cursor, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.arrow_bgm, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.arrow_sfx, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.arrow_lang, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_title, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_bgm_title, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_bgm_value, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_sfx_title, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_sfx_value, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_lang_title, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_lang_value, "/animation/options/content-close.yml"),
            self.cx
                .play_animation(self.text_close, "/animation/options/content-close.yml"),
        )
        .expect("animation not found");

        trace!("Finish Options close animation");
    }

    async fn start(&self, global_data: &mut game::GlobalData) {
        self.cx.play_sfx("/audio/sfx/menu.ogg");
        self.cx
            .set_text_key(
                self.text_bgm_value,
                Self::volume_to_key(global_data.save_data().bgm_volume()),
            )
            .unwrap();
        self.cx
            .set_text_key(
                self.text_sfx_value,
                Self::volume_to_key(global_data.save_data().sfx_volume()),
            )
            .unwrap();
        self.enter_animation().await;

        let mut index = 0;
        loop {
            match index {
                0 => {
                    self.cx
                        .set_position(self.cursor, 200.0, 150.0, 405)
                        .unwrap();
                }
                1 => {
                    self.cx
                        .set_position(self.cursor, 200.0 - 0.17632698 * 130.0, 280.0, 405)
                        .unwrap();
                }
                2 => {
                    self.cx
                        .set_position(self.cursor, 200.0 - 0.17632698 * 130.0 * 2.0, 410.0, 405)
                        .unwrap();
                }
                3 => {
                    self.cx
                        .set_position(self.cursor, 200.0 - 0.17632698 * 130.0 * 3.0, 540.0, 405)
                        .unwrap();
                }
                _ => unreachable!(),
            }
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    index = (index - 1 + 4) % 4;
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                }
                _ = input::wait_down(self.cx).fuse() => {
                    index = (index + 1 + 4) % 4;
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                }
                _ = input::wait_left(self.cx).fuse() => {
                    match index {
                        0 => {
                            self.cx.play_sfx("/audio/sfx/cursor.ogg");
                            global_data.save_data().bgm_volume_down()
                                .expect("Failed to down bgm volume");
                            self.cx
                                .set_text_key(
                                    self.text_bgm_value,
                                    Self::volume_to_key(global_data.save_data().bgm_volume()),
                                )
                                .unwrap();
                        },
                        1 => {
                            self.cx.play_sfx("/audio/sfx/cursor.ogg");
                            global_data.save_data().sfx_volume_down()
                                .expect("Failed to down sfx volume");
                            self.cx
                                .set_text_key(
                                    self.text_sfx_value,
                                    Self::volume_to_key(global_data.save_data().sfx_volume()),
                                )
                                .unwrap();
                        },
                        2 => {
                            self.cx.play_sfx("/audio/sfx/cursor.ogg");
                            let len = localization::len();
                            let lang = global_data.save_data().language();
                            let lang = (lang + len - 1) % len;
                            global_data.save_data().set_language(lang)
                                .expect("Failed to change language");
                        }
                        3 => (),
                        _ => unreachable!(),
                    }
                }
                _ = input::wait_right(self.cx).fuse() => {
                    match index {
                        0 => {
                            self.cx.play_sfx("/audio/sfx/cursor.ogg");
                            global_data.save_data().bgm_volume_up()
                                .expect("Failed to down bgm volume");
                            self.cx
                                .set_text_key(
                                    self.text_bgm_value,
                                    Self::volume_to_key(global_data.save_data().bgm_volume()),
                                )
                                .unwrap();
                        },
                        1 => {
                            self.cx.play_sfx("/audio/sfx/cursor.ogg");
                            global_data.save_data().sfx_volume_up()
                                .expect("Failed to down sfx volume");
                            self.cx
                                .set_text_key(
                                    self.text_sfx_value,
                                    Self::volume_to_key(global_data.save_data().sfx_volume()),
                                )
                                .unwrap();
                        },
                        2 => {
                            self.cx.play_sfx("/audio/sfx/cursor.ogg");
                            let len = localization::len();
                            let lang = global_data.save_data().language();
                            let lang = (lang + len + 1) % len;
                            global_data.save_data().set_language(lang)
                                .expect("Failed to change language");
                        }
                        3 => (),
                        _ => unreachable!(),
                    }
                }
                _ = input::wait_select_button(self.cx).fuse() => {
                    if index == 3 {
                        break;
                    }
                }
                _ = input::wait_cancel_button(self.cx).fuse() => break,
            }
            global_data.save_data().apply(self.cx);
            next_frame().await;
        }

        self.cx.play_sfx("/audio/sfx/cancel.ogg");
        self.close_animation().await;
    }
}
impl<'a> Drop for Options<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.bg_cover);
        self.cx.delete_entity(self.cursor);
        self.cx.delete_entity(self.arrow_bgm);
        self.cx.delete_entity(self.arrow_sfx);
        self.cx.delete_entity(self.arrow_lang);
        self.cx.delete_entity(self.text_title);
        self.cx.delete_entity(self.text_bgm_title);
        self.cx.delete_entity(self.text_bgm_value);
        self.cx.delete_entity(self.text_sfx_title);
        self.cx.delete_entity(self.text_sfx_value);
        self.cx.delete_entity(self.text_lang_title);
        self.cx.delete_entity(self.text_lang_value);
        self.cx.delete_entity(self.text_close);
    }
}

pub async fn options(cx: &AnimationEngineContext, global_data: &mut game::GlobalData) {
    info!("Enter Options");
    Options::new(cx).start(global_data).await;
}
