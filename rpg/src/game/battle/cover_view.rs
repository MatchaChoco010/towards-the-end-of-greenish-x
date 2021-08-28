use animation_engine::*;
use futures::try_join;

pub(super) struct CoverView<'a> {
    cx: &'a AnimationEngineContext,
    part_12: Entity,
    part_13: Entity,
    part_14: Entity,
    cover: Entity,
}
impl<'a> CoverView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_12 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-12.png".into(),
            y: -720.0,
            z: 502,
            ..Default::default()
        });
        let part_13 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-13.png".into(),
            y: -720.0,
            z: 501,
            ..Default::default()
        });
        let part_14 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-14.png".into(),
            y: -720.0,
            z: 500,
            ..Default::default()
        });
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 503,
            ..Default::default()
        });
        Self {
            cx,
            part_12,
            part_13,
            part_14,
            cover,
        }
    }

    pub(super) async fn fade_out(&self) {
        self.cx.set_opacity(self.part_12, 0.0).unwrap();
        self.cx.set_opacity(self.part_13, 0.0).unwrap();
        self.cx.set_opacity(self.part_14, 0.0).unwrap();
        self.cx
            .play_animation(self.cover, "/animation/battle/cover-fade-out.yml")
            .await
            .expect("animation not found");
    }

    pub(super) async fn start_battle(&self) {
        self.cx.set_opacity(self.cover, 1.0).unwrap();
        self.cx.set_opacity(self.part_12, 1.0).unwrap();
        self.cx.set_opacity(self.part_13, 1.0).unwrap();
        self.cx.set_opacity(self.part_14, 1.0).unwrap();
        try_join!(
            self.cx
                .play_animation(self.cover, "/animation/battle/cover-battle-start.yml"),
            self.cx
                .play_animation(self.part_12, "/animation/battle/cover-battle-start-1.yml"),
            self.cx
                .play_animation(self.part_13, "/animation/battle/cover-battle-start-2.yml"),
            self.cx
                .play_animation(self.part_14, "/animation/battle/cover-battle-start-3.yml"),
        )
        .expect("animation not found");
    }
}
impl<'a> Drop for CoverView<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_12);
        self.cx.delete_entity(self.part_13);
        self.cx.delete_entity(self.part_14);
        self.cx.delete_entity(self.cover);
    }
}
