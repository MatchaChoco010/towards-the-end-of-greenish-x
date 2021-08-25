use animation_engine::*;
use futures::try_join;

pub(super) struct MessageWindowView<'a> {
    cx: &'a AnimationEngineContext,
    part_0: Entity,
    part_1: Entity,
    text: Entity,
    turn_text: Entity,
}
impl<'a> MessageWindowView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-0.png".into(),
            z: 310,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-1.png".into(),
            z: 350,
            ..Default::default()
        });
        let text = cx.add_text(AddTextInfo {
            font_size: 36.0,
            x: 120.0,
            z: 360,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let turn_text = cx.add_text(AddTextInfo {
            key: "battle-message-turns".into(),
            font_size: 36.0,
            x: 60.0,
            y: 60.0,
            z: 360,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        cx.set_text_format_args(turn_text, &["1"]).unwrap();
        Self {
            cx,
            part_0,
            part_1,
            text,
            turn_text,
        }
    }

    pub(super) async fn start_battle(&self) {
        try_join!(
            self.cx.play_animation(
                self.part_0,
                "/animation/battle/message-window-battle-start-0.yml"
            ),
            self.cx.play_animation(
                self.part_1,
                "/animation/battle/message-window-battle-start-1.yml"
            ),
            self.cx.play_animation(
                self.turn_text,
                "/animation/battle/message-window-battle-start-2.yml"
            ),
        )
        .expect("animation not found");
    }

    pub(super) async fn add_message(&self, key: impl ToString, args: &[&str]) {
        self.cx
            .play_animation(self.text, "/animation/battle/message-fade-out.yml")
            .await
            .expect("animation not found");
        self.cx.set_text_key(self.text, key).unwrap();
        self.cx.set_text_format_args(self.text, args).unwrap();
        self.cx.play_sfx("/audio/sfx/cursor.ogg");
        self.cx
            .play_animation(self.text, "/animation/battle/message-fade-in.yml")
            .await
            .expect("animation not found");
    }

    pub(super) fn set_turns(&self, turns: u32) {
        self.cx
            .set_text_format_args(self.turn_text, &[&turns.to_string()])
            .unwrap();
    }
}
impl<'a> Drop for MessageWindowView<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.text);
        self.cx.delete_entity(self.turn_text);
    }
}
