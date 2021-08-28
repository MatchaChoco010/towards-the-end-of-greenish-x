use animation_engine::*;
use futures::future::try_join_all;

#[derive(Clone, Copy, Debug)]
pub(super) enum Number {
    Number(i32),
    Infinity,
}

pub(super) struct NumberView<'a> {
    cx: &'a AnimationEngineContext,
    numbers: Vec<Entity>,
    x: f32,
    y: f32,
    z: u32,
}
impl<'a> NumberView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext, x: f32, y: f32, z: u32) -> Self {
        let numbers = vec![];
        Self {
            cx,
            numbers,
            x,
            y,
            z,
        }
    }

    pub(super) fn set_number(&mut self, number: Number) {
        for number_entity in self.numbers.drain(0..) {
            self.cx.delete_entity(number_entity);
        }

        if let Number::Number(number) = number {
            let mut n = number;
            loop {
                let digit = n % 10;

                let number_entity = self.cx.add_image(AddImageInfo {
                    name: format!("/image/number/{}.png", digit),
                    ..Default::default()
                });
                self.numbers.push(number_entity);

                n = n / 10;
                if n == 0 {
                    break;
                }
            }

            for (i, &number_entity) in self.numbers.iter().enumerate() {
                let x = self.x + (i + 1) as f32 * -20.0;
                let y = self.y - 18.0;
                self.cx.set_position(number_entity, x, y, self.z).unwrap();
            }
        } else {
            let entity = self.cx.add_image(AddImageInfo {
                name: "/image/number/8.png".into(),
                x: self.x,
                y: self.y - 10.0,
                z: self.z,
                rotation: std::f32::consts::PI / 2.0,
                ..Default::default()
            });
            self.numbers.push(entity);
        }
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

    pub(super) fn set_opacity(&self, a: f32) {
        self.numbers
            .iter()
            .for_each(|&num| self.cx.set_opacity(num, a).unwrap());
    }
}
impl<'a> Drop for NumberView<'a> {
    fn drop(&mut self) {
        for &number_entity in self.numbers.iter() {
            self.cx.delete_entity(number_entity);
        }
    }
}
