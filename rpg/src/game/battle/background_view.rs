use animation_engine::*;

use crate::game_data;

pub(super) struct BackgroundView<'a> {
    cx: &'a AnimationEngineContext,
    bg: Entity,
    morning_cover: Entity,
    night_cover: Entity,
}
impl<'a> BackgroundView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext, time: game_data::BattleTime) -> Self {
        let bg = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-bg.png".into(),
            x: 0.0,
            z: 300,
            ..Default::default()
        });
        let morning_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 233.0 / 255.0,
            b: 1.0,
            a: 0.0,
            z: 301,
            ..Default::default()
        });
        let night_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 28.0 / 255.0,
            b: 193.0 / 255.0,
            a: 0.0,
            z: 301,
            ..Default::default()
        });
        match time {
            game_data::BattleTime::Morning => cx.set_opacity(morning_cover, 0.25).unwrap(),
            game_data::BattleTime::Afternoon => (),
            game_data::BattleTime::Night => cx.set_opacity(night_cover, 0.25).unwrap(),
        }
        Self {
            cx,
            bg,
            morning_cover,
            night_cover,
        }
    }

    pub(super) async fn start(&self) {
        self.cx
            .play_animation(self.bg, "/animation/battle/start-bg.yml")
            .await
            .expect("animation not found");
    }
}
impl<'a> Drop for BackgroundView<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.morning_cover);
        self.cx.delete_entity(self.night_cover);
    }
}
