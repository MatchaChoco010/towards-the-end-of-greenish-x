use animation_engine_macro::*;
use uuid::Uuid;

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
    pub struct UniformScale {
        #[animation_property]
        pub scale: f32,
    }

    #[derive(Clone, Copy)]
    #[animation_component]
    pub struct Rotation {
        #[animation_property]
        pub rotation: f32,
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
    #[animation_component]
    pub struct Opacity {
        #[animation_property]
        pub opacity: f32,
    }

    #[derive(Clone)]
    pub enum Renderable {
        #[animation_component]
        Rect {
            #[animation_property]
            width: f32,
            #[animation_property]
            height: f32,
        },
        Text {
            text: String,
            font_size: f32,
        },
        Image {
            image: Uuid,
        }
    }

}
