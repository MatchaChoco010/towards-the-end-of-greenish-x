use animation_engine::*;

pub(super) struct Background<'a> {
    cx: &'a AnimationEngineContext,
    bg: Entity,
    morning_cover: Entity,
    night_cover: Entity,
}
impl<'a> Background<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let bg = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-bg.png".into(),
            x: 0.0,
            z: 0,
            ..Default::default()
        });
        let morning_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 233.0 / 255.0,
            b: 1.0,
            a: 0.25,
            z: 5,
            ..Default::default()
        });
        let night_cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 28.0 / 255.0,
            b: 193.0 / 255.0,
            a: 0.0,
            z: 5,
            ..Default::default()
        });
        Self {
            cx,
            bg,
            morning_cover,
            night_cover,
        }
    }

    pub(super) async fn change_to_afternoon(&self) {
        self.cx
            .play_animation(
                self.morning_cover,
                "/animation/explore/morning-cover-out.yml",
            )
            .await
            .expect("animation not found");
    }

    pub(super) async fn change_to_night(&self) {
        self.cx
            .play_animation(self.night_cover, "/animation/explore/night-cover-in.yml")
            .await
            .expect("animation not found");
    }
}
impl<'a> Drop for Background<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.bg);
        self.cx.delete_entity(self.morning_cover);
        self.cx.delete_entity(self.night_cover);
    }
}
