use animation_engine::*;
use futures::future::try_join_all;

pub(super) struct DamageNumberView<'a> {
    cx: &'a AnimationEngineContext,
    numbers: Vec<Entity>,
}
impl<'a> DamageNumberView<'a> {
    pub(super) fn new_damage(
        cx: &'a AnimationEngineContext,
        number: i32,
        x: f32,
        y: f32,
        z: u32,
    ) -> Self {
        Self::new(cx, number, x, y, z, "damage-")
    }
    pub(super) fn new_heal(
        cx: &'a AnimationEngineContext,
        number: i32,
        x: f32,
        y: f32,
        z: u32,
    ) -> Self {
        Self::new(cx, number, x, y, z, "heal-")
    }

    fn new(
        cx: &'a AnimationEngineContext,
        number: i32,
        x: f32,
        y: f32,
        z: u32,
        prefix: &str,
    ) -> Self {
        let mut numbers = vec![];
        let mut n = number;
        loop {
            let digit = n % 10;

            let number_entity = cx.add_image(AddImageInfo {
                name: format!("/image/number/{}{}.png", prefix, digit),
                ..Default::default()
            });
            numbers.push(number_entity);

            n = n / 10;
            if n == 0 {
                break;
            }
        }

        let len = numbers.len();
        for (i, &number_entity) in numbers.iter().enumerate() {
            let x = x + i as f32 * -20.0 + len as f32 * 0.5 * 20.0;
            let y = y - 18.0;
            cx.set_position(number_entity, x, y, z).unwrap();
        }

        Self { cx, numbers }
    }

    pub(super) async fn start_animation(&self, animation: &str) {
        try_join_all(
            self.numbers
                .iter()
                .map(|&n| self.cx.play_animation(n, animation)),
        )
        .await
        .expect("animation not found");
    }
}
impl<'a> Drop for DamageNumberView<'a> {
    fn drop(&mut self) {
        for &number_entity in self.numbers.iter() {
            self.cx.delete_entity(number_entity);
        }
    }
}
