use animation_engine::*;

pub(super) struct WindowFrame<'a> {
    cx: &'a AnimationEngineContext,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
}
impl<'a> WindowFrame<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-0.png".into(),
            z: 10,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-1.png".into(),
            z: 10,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-2.png".into(),
            x: 1280.0 - 148.0,
            z: 10,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-3.png".into(),
            x: 1280.0 - 105.0,
            y: 720.0 - 592.0,
            z: 10,
            ..Default::default()
        });
        Self {
            cx,
            part_0,
            part_1,
            part_2,
            part_3,
        }
    }
}
impl<'a> Drop for WindowFrame<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
    }
}
