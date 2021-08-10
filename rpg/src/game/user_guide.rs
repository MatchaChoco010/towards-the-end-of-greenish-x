use animation_engine::*;
use futures::{select, try_join, FutureExt};
use log::{info, trace};

use crate::input;

struct UserGuide<'a> {
    cx: &'a AnimationEngineContext,
    bg_cover: Entity,
    bg: Entity,
    game_pad_img: Entity,
    zxc_img: Entity,
    arrow_img: Entity,
    text_title: Entity,
    text_1: Entity,
    text_2: Entity,
    text_3: Entity,
    text_4: Entity,
    text_5: Entity,
    text_6: Entity,
    text_z: Entity,
    text_c: Entity,
    text_x: Entity,
    text_arrow: Entity,
}
impl<'a> UserGuide<'a> {
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
        let game_pad_img = cx.add_image(AddImageInfo {
            name: "/image/ui/user-guide-game-pad.png".into(),
            x: 120.0,
            y: 130.0,
            z: 410,
            ..Default::default()
        });
        let zxc_img = cx.add_image(AddImageInfo {
            name: "/image/ui/user-guide-zxc.png".into(),
            x: 160.0,
            y: 500.0,
            z: 410,
            ..Default::default()
        });
        let arrow_img = cx.add_image(AddImageInfo {
            name: "/image/ui/user-guide-arrow.png".into(),
            x: 620.0,
            y: 550.0,
            z: 410,
            ..Default::default()
        });
        let text_title = cx.add_text(AddTextInfo {
            key: "user-guide-title".into(),
            font_size: 72.0,
            x: 500.0,
            y: 30.0,
            z: 410,
            ..Default::default()
        });
        let text_1 = cx.add_text(AddTextInfo {
            key: "user-guide-1".into(),
            font_size: 26.0,
            x: 685.0,
            y: 135.0,
            z: 410,
            ..Default::default()
        });
        let text_2 = cx.add_text(AddTextInfo {
            key: "user-guide-2".into(),
            font_size: 26.0,
            x: 685.0,
            y: 183.0,
            z: 410,
            ..Default::default()
        });
        let text_3 = cx.add_text(AddTextInfo {
            key: "user-guide-3".into(),
            font_size: 26.0,
            x: 685.0,
            y: 231.0,
            z: 410,
            ..Default::default()
        });
        let text_4 = cx.add_text(AddTextInfo {
            key: "user-guide-4".into(),
            font_size: 26.0,
            x: 685.0,
            y: 279.0,
            z: 410,
            ..Default::default()
        });
        let text_5 = cx.add_text(AddTextInfo {
            key: "user-guide-5".into(),
            font_size: 26.0,
            x: 685.0,
            y: 327.0,
            z: 410,
            ..Default::default()
        });
        let text_6 = cx.add_text(AddTextInfo {
            key: "user-guide-6".into(),
            font_size: 26.0,
            x: 685.0,
            y: 375.0,
            z: 410,
            ..Default::default()
        });
        let text_z = cx.add_text(AddTextInfo {
            key: "user-guide-z".into(),
            font_size: 26.0,
            x: 230.0,
            y: 510.0,
            z: 410,
            ..Default::default()
        });
        let text_x = cx.add_text(AddTextInfo {
            key: "user-guide-x".into(),
            font_size: 26.0,
            x: 230.0,
            y: 570.0,
            z: 410,
            ..Default::default()
        });
        let text_c = cx.add_text(AddTextInfo {
            key: "user-guide-c".into(),
            font_size: 26.0,
            x: 230.0,
            y: 630.0,
            z: 410,
            ..Default::default()
        });
        let text_arrow = cx.add_text(AddTextInfo {
            key: "user-guide-arrow".into(),
            font_size: 26.0,
            x: 780.0,
            y: 615.0,
            z: 410,
            ..Default::default()
        });
        Self {
            cx,
            bg_cover,
            bg,
            game_pad_img,
            zxc_img,
            arrow_img,
            text_title,
            text_1,
            text_2,
            text_3,
            text_4,
            text_5,
            text_6,
            text_z,
            text_x,
            text_c,
            text_arrow,
        }
    }

    async fn start(&self) {
        self.cx.play_sfx("/audio/sfx/menu.ogg");

        trace!("start user guide enter animation");

        try_join!(
            self.cx
                .play_animation(self.bg_cover, "/animation/user-guide/cover-enter.yml"),
            self.cx
                .play_animation(self.bg, "/animation/user-guide/bg-enter.yml"),
            self.cx
                .play_animation(self.game_pad_img, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.zxc_img, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.arrow_img, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_title, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_1, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_2, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_3, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_4, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_5, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_6, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_z, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_x, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_c, "/animation/user-guide/content-enter.yml"),
            self.cx
                .play_animation(self.text_arrow, "/animation/user-guide/content-enter.yml"),
        )
        .expect("animation not found");

        trace!("finish user guide enter animation");

        select! {
            _ = input::wait_select_button(self.cx).fuse() => (),
            _ = input::wait_cancel_button(self.cx).fuse() => (),
        }

        self.cx.play_sfx("/audio/sfx/cancel.ogg");

        trace!("start user guide close animation");

        try_join!(
            self.cx
                .play_animation(self.bg_cover, "/animation/user-guide/cover-close.yml"),
            self.cx
                .play_animation(self.bg, "/animation/user-guide/bg-close.yml"),
            self.cx
                .play_animation(self.game_pad_img, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.zxc_img, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.arrow_img, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_title, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_1, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_2, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_3, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_4, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_5, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_6, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_z, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_x, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_c, "/animation/user-guide/content-close.yml"),
            self.cx
                .play_animation(self.text_arrow, "/animation/user-guide/content-close.yml"),
        )
        .expect("animation not found");

        trace!("finish user guide close animation");
    }
}
impl<'a> Drop for UserGuide<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.bg_cover);
        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.game_pad_img);
        self.cx.delete_entity(self.zxc_img);
        self.cx.delete_entity(self.arrow_img);
        self.cx.delete_entity(self.text_title);
        self.cx.delete_entity(self.text_1);
        self.cx.delete_entity(self.text_2);
        self.cx.delete_entity(self.text_3);
        self.cx.delete_entity(self.text_4);
        self.cx.delete_entity(self.text_5);
        self.cx.delete_entity(self.text_6);
        self.cx.delete_entity(self.text_z);
        self.cx.delete_entity(self.text_x);
        self.cx.delete_entity(self.text_c);
        self.cx.delete_entity(self.text_arrow);
    }
}

pub async fn user_guide(cx: &AnimationEngineContext) {
    info!("Enter UserGuide Scene!");
    UserGuide::new(cx).start().await;
}
