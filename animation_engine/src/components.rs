#[derive(Clone, Copy)]
pub(crate) struct Position {
    pub x: f32,
    pub y: f32,
    pub z: u32,
}

#[derive(Clone, Copy)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Copy)]
pub(crate) enum Renderable {
    Rect { width: f32, height: f32 },
}
