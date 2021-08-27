use animation_engine::*;
use futures::try_join;

pub(super) struct Cover<'a> {
    cx: &'a AnimationEngineContext,
    part_24: Entity,
    part_25: Entity,
    part_26: Entity,
    cover: Entity,
}
impl<'a> Cover<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-24.png".into(),
            y: 720.0,
            z: 200,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-25.png".into(),
            y: 720.0,
            z: 199,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-26.png".into(),
            y: 720.0,
            z: 198,
            ..Default::default()
        });
        let part_3 = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 200,
            ..Default::default()
        });
        Self {
            cx,
            part_24: part_0,
            part_25: part_1,
            part_26: part_2,
            cover: part_3,
        }
    }

    pub(super) async fn fade_in(&self) {
        self.cx.set_opacity(self.part_24, 0.0).unwrap();
        self.cx.set_opacity(self.part_25, 0.0).unwrap();
        self.cx.set_opacity(self.part_26, 0.0).unwrap();
        self.cx
            .play_animation(self.cover, "/animation/explore/cover-fade-in.yml")
            .await
            .expect("animation not found");
    }

    pub(super) async fn fade_out(&self) {
        self.cx.set_opacity(self.part_24, 0.0).unwrap();
        self.cx.set_opacity(self.part_25, 0.0).unwrap();
        self.cx.set_opacity(self.part_26, 0.0).unwrap();
        self.cx
            .play_animation(self.cover, "/animation/explore/cover-fade-out.yml")
            .await
            .expect("animation not found");
    }

    pub(super) async fn start_battle(&self) {
        self.cx.set_opacity(self.part_24, 1.0).unwrap();
        self.cx.set_opacity(self.part_25, 1.0).unwrap();
        self.cx.set_opacity(self.part_26, 1.0).unwrap();
        try_join!(
            self.cx
                .play_animation(self.cover, "/animation/explore/cover-battle-start.yml"),
            self.cx
                .play_animation(self.part_24, "/animation/explore/cover-battle-start-1.yml"),
            self.cx
                .play_animation(self.part_25, "/animation/explore/cover-battle-start-2.yml"),
            self.cx
                .play_animation(self.part_26, "/animation/explore/cover-battle-start-3.yml"),
        )
        .expect("animation not found");
    }
}
impl<'a> Drop for Cover<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_24);
        self.cx.delete_entity(self.part_25);
        self.cx.delete_entity(self.part_26);
        self.cx.delete_entity(self.cover);
    }
}
