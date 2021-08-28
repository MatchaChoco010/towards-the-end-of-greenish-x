use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, try_join};

use crate::game::battle::damage_number_view::*;

pub(super) struct PlayerView<'a> {
    cx: &'a AnimationEngineContext,
    player_image: Entity,
    player_shadow_image: Entity,
    part_7: Entity,
    part_8: Entity,
    hp_title: Entity,
    hp_text: Entity,
    hp_bar: Entity,
    tp_title: Entity,
    tp_text: Entity,
    tp_bar: Entity,
}
impl<'a> PlayerView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext, player_index: usize) -> Self {
        let player_image = cx.add_image(AddImageInfo {
            name: format!("/image/player/player-{}.png", player_index),
            z: 470,
            ..Default::default()
        });
        let player_shadow_image = cx.add_image(AddImageInfo {
            name: format!("/image/player/player-{}-shadow.png", player_index),
            z: 465,
            ..Default::default()
        });
        let part_7 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-7.png".into(),
            x: 1040.0,
            y: 478.0,
            z: 475,
            ..Default::default()
        });
        let part_8 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-8.png".into(),
            x: 1040.0,
            y: 478.0,
            z: 485,
            ..Default::default()
        });
        let hp_title = cx.add_text(AddTextInfo {
            key: "battle-hp-title".into(),
            font_size: 36.0,
            x: 1086.0,
            y: 560.0,
            z: 490,
            rotation: -0.0872665,
            ..Default::default()
        });
        let hp_text = cx.add_text(AddTextInfo {
            key: "battle-hp-text".into(),
            font_size: 24.0,
            format_args: vec![format!("{:4}", 150), "150".into()],
            x: 1152.0,
            y: 566.0,
            z: 490,
            rotation: -0.0872665,
            ..Default::default()
        });
        let hp_bar = cx.add_rect(AddRectInfo {
            width: 180.0,
            height: 50.0,
            x: 1080.0,
            y: 575.0,
            r: 0.0,
            g: 234.0 / 255.0,
            b: 1.0,
            z: 480,
            ..Default::default()
        });
        let tp_title = cx.add_text(AddTextInfo {
            key: "battle-tp-title".into(),
            font_size: 36.0,
            x: 1078.0,
            y: 637.0,
            z: 490,
            rotation: -0.0872665,
            ..Default::default()
        });
        let tp_text = cx.add_text(AddTextInfo {
            key: "battle-tp-text".into(),
            font_size: 24.0,
            format_args: vec![format!("{:3}", 50), "50".into()],
            x: 1160.0,
            y: 642.0,
            z: 490,
            rotation: -0.0872665,
            ..Default::default()
        });
        let tp_bar = cx.add_rect(AddRectInfo {
            width: 180.0,
            height: 50.0,
            x: 1075.0,
            y: 648.0,
            r: 1.0,
            g: 64.0 / 255.0,
            b: 0.0,
            z: 480,
            ..Default::default()
        });
        Self {
            cx,
            player_image,
            player_shadow_image,
            part_7,
            part_8,
            hp_title,
            hp_text,
            hp_bar,
            tp_title,
            tp_text,
            tp_bar,
        }
    }

    pub(super) async fn start_battle(&self) {
        try_join!(
            self.cx.play_animation(
                self.player_image,
                "/animation/battle/player-image-battle-start.yml"
            ),
            self.cx.play_animation(
                self.player_shadow_image,
                "/animation/battle/player-shadow-image-battle-start.yml"
            ),
        )
        .expect("animation not found");
    }

    pub(super) fn set_hp(&self, hp: i32, max_hp: i32) {
        self.cx
            .set_text_format_args(self.hp_text, &[&format!("{:4}", hp), &max_hp.to_string()])
            .unwrap();
        self.cx
            .set_width(
                self.hp_bar,
                180.0 * (hp.min(max_hp) as f32 / max_hp as f32 + 0.00001),
            )
            .unwrap();
    }

    pub(super) fn set_tp(&self, tp: i32, max_tp: i32) {
        self.cx
            .set_text_format_args(self.tp_text, &[&format!("{:4}", tp), &max_tp.to_string()])
            .unwrap();
        self.cx
            .set_width(
                self.tp_bar,
                180.0 * (tp.min(max_tp) as f32 / max_tp as f32 + 0.00001),
            )
            .unwrap();
    }

    pub(super) fn damage_animation(&self, damage: i32) {
        let cx = self.cx.clone();
        let player_image = self.player_image;
        let player_shadow_image = self.player_shadow_image;
        spawn(async move {
            let damage = DamageNumberView::new_damage(&cx, damage, 1160.0, 570.0, 495);
            let result = join!(
                cx.play_animation(player_image, "/animation/battle/player-image-damage.yml"),
                cx.play_animation(
                    player_shadow_image,
                    "/animation/battle/player-shadow-image-damage.yml"
                ),
                damage.start_animation("/animation/battle/player-damage-number-animation.yml"),
            );
            result.0.expect("animation not found");
            result.1.expect("animation not found");
        });
    }

    pub(super) fn heal_animation(&self, heal: i32) {
        let cx = self.cx.clone();
        spawn(async move {
            DamageNumberView::new_heal(&cx, heal, 1160.0, 570.0, 495)
                .start_animation("/animation/battle/player-damage-number-animation.yml")
                .await;
        });
    }

    pub(super) async fn blink_animation(&self) {
        self.cx
            .play_animation(self.player_image, "/animation/battle/blink.yml")
            .await
            .expect("animation not found");
    }

    pub(super) async fn blink_animation_loop(&self) {
        loop {
            for i in 0..60 {
                let overlay = 1.0
                    - if i < 30 {
                        i as f32 / 30.0
                    } else {
                        (60 - i) as f32 / 30.0
                    };
                self.cx
                    .set_color(self.player_image, overlay, overlay, overlay)
                    .unwrap();
                next_frame().await;
            }
        }
    }

    pub(super) fn reset_blink(&self) {
        self.cx.set_color(self.player_image, 1.0, 1.0, 1.0).unwrap();
    }
}
impl<'a> Drop for PlayerView<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.player_image);
        self.cx.delete_entity(self.player_shadow_image);
        self.cx.delete_entity(self.part_7);
        self.cx.delete_entity(self.part_8);
        self.cx.delete_entity(self.hp_title);
        self.cx.delete_entity(self.hp_text);
        self.cx.delete_entity(self.hp_bar);
        self.cx.delete_entity(self.tp_title);
        self.cx.delete_entity(self.tp_text);
        self.cx.delete_entity(self.tp_bar);
    }
}
