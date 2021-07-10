use animation_engine_macro::*;

anim_components! {

    #[derive(Clone, Copy)]
    #[animation_component]
    pub struct Position {
        #[animation_property]
        pub x: f32,
        #[animation_property]
        pub y: f32,
        pub z: u32,
    }

    #[derive(Clone, Copy)]
    #[animation_component]
    pub struct Color {
        #[animation_property]
        pub r: f32,
        #[animation_property]
        pub g: f32,
        #[animation_property]
        pub b: f32,
    }

    #[derive(Clone, Copy)]
    pub enum Renderable {
        #[animation_component]
        Rect {
            #[animation_property]
            width: f32,
            #[animation_property]
            height: f32,
        }
    }

}
