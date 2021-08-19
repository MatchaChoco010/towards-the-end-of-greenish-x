use animation_engine::*;
pub(super) struct CurrentDepth<'a> {
    cx: &'a AnimationEngineContext,
    text: Entity,
    count_text: Entity,
    depth: u32,
    max_depth: u32,
}
impl<'a> CurrentDepth<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let text = cx.add_text(AddTextInfo {
            key: "explore-depth".into(),
            font_size: 48.0,
            x: 40.0,
            y: 40.0,
            z: 20,
            ..Default::default()
        });
        let count_text = cx.add_text(AddTextInfo {
            key: "explore-current-depth".into(),
            font_size: 36.0,
            format_args: vec!["  1".into(), "1".into()],
            x: 70.0,
            y: 90.0,
            z: 20,
            ..Default::default()
        });
        Self {
            cx,
            text,
            count_text,
            depth: 1,
            max_depth: 1,
        }
    }

    pub(super) fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth as u32;
        self.cx
            .set_text_format_args(
                self.count_text,
                &[&format!("{:3}", self.depth), &format!("{}", self.max_depth)],
            )
            .unwrap();
    }

    pub(super) fn increment(&mut self) {
        self.depth += 1;
        self.cx
            .set_text_format_args(
                self.count_text,
                &[&format!("{:3}", self.depth), &format!("{}", self.max_depth)],
            )
            .unwrap();
    }
}
impl<'a> Drop for CurrentDepth<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.text);
        self.cx.delete_entity(self.count_text);
    }
}
