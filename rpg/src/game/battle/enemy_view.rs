use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, try_join};

use crate::game::battle::damage_number_view::*;

pub(super) struct EnemyView<'a> {
    cx: &'a AnimationEngineContext,
    enemy_image: Entity,
    enemy_shadow_image: Entity,
    part_5: Entity,
    part_6: Entity,
    hp_bar: Entity,
}
impl<'a> EnemyView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let enemy_image = cx.add_image(AddImageInfo {
            x: 235.0,
            z: 340,
            ..Default::default()
        });
        let enemy_shadow_image = cx.add_image(AddImageInfo {
            x: 235.0,
            z: 335,
            ..Default::default()
        });
        let part_5 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-5.png".into(),
            x: 346.0,
            z: 355,
            ..Default::default()
        });
        let part_6 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-6.png".into(),
            x: 346.0,
            z: 360,
            ..Default::default()
        });
        let hp_bar = cx.add_rect(AddRectInfo {
            width: 285.0,
            height: 35.0,
            x: 345.0,
            z: 357,
            r: 1.0,
            g: 0.0,
            b: 70.0 / 255.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        Self {
            cx,
            enemy_image,
            enemy_shadow_image,
            part_5,
            part_6,
            hp_bar,
        }
    }

    pub(super) fn set_monster_image(&self, image: impl ToString, shadow_image: impl ToString) {
        self.cx.set_image_name(self.enemy_image, image).unwrap();
        self.cx
            .set_image_name(self.enemy_shadow_image, shadow_image)
            .unwrap();
    }

    pub(super) async fn start_battle(&self) {
        try_join!(
            self.cx.play_animation(
                self.enemy_image,
                "/animation/battle/enemy-image-battle-start.yml"
            ),
            self.cx.play_animation(
                self.enemy_shadow_image,
                "/animation/battle/enemy-shadow-image-battle-start.yml"
            ),
            self.cx
                .play_animation(self.part_5, "/animation/battle/enemy-hp-battle-start.yml"),
            self.cx
                .play_animation(self.part_6, "/animation/battle/enemy-hp-battle-start.yml"),
            self.cx.play_animation(
                self.hp_bar,
                "/animation/battle/enemy-hp-bar-battle-start.yml"
            ),
        )
        .expect("animation not found");
    }

    pub(super) async fn down_enemy(&self) {
        try_join!(
            self.cx
                .play_animation(self.enemy_image, "/animation/battle/enemy-image-down.yml"),
            self.cx.play_animation(
                self.enemy_shadow_image,
                "/animation/battle/enemy-shadow-image-down.yml"
            ),
            self.cx
                .play_animation(self.part_5, "/animation/battle/enemy-hp-down.yml"),
            self.cx
                .play_animation(self.part_6, "/animation/battle/enemy-hp-down.yml"),
            self.cx
                .play_animation(self.hp_bar, "/animation/battle/enemy-hp-bar-down.yml"),
        )
        .expect("animation not found");
    }

    // pub(super) async fn start_battle_boss(&self)

    pub(super) fn set_hp(&self, hp: i32, max_hp: i32) {
        self.cx
            .set_width(
                self.hp_bar,
                285.0 * (hp.min(max_hp) as f32 / max_hp as f32 + 0.00001),
            )
            .unwrap();
    }

    pub(super) fn damage_animation(&self, damage: i32) {
        let cx = self.cx.clone();
        let enemy_image = self.enemy_image;
        let enemy_shadow_image = self.enemy_shadow_image;
        spawn(async move {
            let damage = DamageNumberView::new_damage(&cx, damage, 500.0, 460.0, 370);
            let result = join!(
                cx.play_animation(enemy_image, "/animation/battle/enemy-damage.yml"),
                cx.play_animation(enemy_shadow_image, "/animation/battle/enemy-damage.yml"),
                damage.start_animation("/animation/battle/enemy-damage-number-animation.yml")
            );
            result.0.expect("animation not found");
            result.1.expect("animation not found");
        });
    }

    pub(super) fn heal_animation(&self, heal: i32) {
        let cx = self.cx.clone();
        spawn(async move {
            DamageNumberView::new_heal(&cx, heal, 500.0, 460.0, 370)
                .start_animation("/animation/battle/enemy-damage-number-animation.yml")
                .await;
        });
    }

    pub(super) async fn blink_animation(&self) {
        self.cx
            .play_animation(self.enemy_image, "/animation/battle/blink.yml")
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
                    .set_color(self.enemy_image, overlay, overlay, overlay)
                    .unwrap();
                next_frame().await;
            }
        }
    }

    pub(super) fn reset_blink(&self) {
        self.cx.set_color(self.enemy_image, 1.0, 1.0, 1.0).unwrap();
    }
}
impl<'a> Drop for EnemyView<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.enemy_image);
        self.cx.delete_entity(self.enemy_shadow_image);
        self.cx.delete_entity(self.part_5);
        self.cx.delete_entity(self.part_6);
        self.cx.delete_entity(self.hp_bar);
    }
}
