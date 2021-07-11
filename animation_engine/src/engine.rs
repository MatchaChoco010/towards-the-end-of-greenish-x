use crate::animation_components::*;
use crate::image_store::*;
use crate::key_input_state::*;
use crate::render::*;
use anyhow::Result;
use ggez::event::{self, EventHandler, EventLoop, KeyCode};
use ggez::*;
use legion::*;
use std::env;
use std::path;
use std::path::Path;
use std::time::{Duration, Instant};

pub use crate::animation_components::AnimationFinishChecker;

pub struct AddRectInfo {
    pub x: f32,
    pub y: f32,
    pub z: u32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub rotation: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Default for AddRectInfo {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0,
            width: 100.0,
            height: 100.0,
            scale: 1.0,
            rotation: 0.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

pub struct AddImageInfo {
    pub x: f32,
    pub y: f32,
    pub z: u32,
    pub scale: f32,
    pub rotation: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub name: String,
}
impl Default for AddImageInfo {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0,
            scale: 1.0,
            rotation: 0.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
            name: "".to_string(),
        }
    }
}

pub struct AddTextInfo {
    pub x: f32,
    pub y: f32,
    pub z: u32,
    pub scale: f32,
    pub rotation: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub text: String,
    pub font_size: f32,
}
impl Default for AddTextInfo {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0,
            scale: 1.0,
            rotation: 0.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
            text: "Hello World!".to_string(),
            font_size: 24.0,
        }
    }
}

pub struct AnimationEngineContext {
    current_time: Instant,
    delta_time: Duration,
    world: World,
    resources: Resources,
    schedule: Schedule,
    update_function: Option<Box<dyn FnMut(&mut AnimationEngineContext) -> ()>>,
    key_input: KeyInputState,
}
impl AnimationEngineContext {
    fn new() -> Self {
        let mut schedule = Schedule::builder();
        add_animation_system(&mut schedule);
        let schedule = schedule.build();

        let mut resources = Resources::default();
        resources.insert::<AnimationStore>(AnimationStore::new());
        resources.insert::<ImageStore>(ImageStore::new());

        Self {
            current_time: Instant::now(),
            delta_time: Duration::new(0, 0),
            world: World::default(),
            resources,
            schedule,
            update_function: None,
            key_input: KeyInputState::new(),
        }
    }

    pub fn add_rect(&mut self, info: AddRectInfo) -> Entity {
        let AddRectInfo {
            x,
            y,
            z,
            width,
            height,
            scale,
            rotation,
            r,
            g,
            b,
            a,
        } = info;
        self.world.push((
            Position { x, y, z },
            UniformScale { scale },
            Rotation { rotation },
            Color { r, g, b },
            Opacity { opacity: a },
            Renderable::Rect { width, height },
        ))
    }

    pub fn add_image(&mut self, info: AddImageInfo) -> Entity {
        let AddImageInfo {
            x,
            y,
            z,
            scale,
            rotation,
            r,
            g,
            b,
            a,
            name,
        } = info;
        let uuid = self
            .resources
            .get::<ImageStore>()
            .unwrap()
            .get_image_uuid_from_name(name)
            .expect("Failed to get image");
        self.world.push((
            Position { x, y, z },
            UniformScale { scale },
            Rotation { rotation },
            Color { r, g, b },
            Opacity { opacity: a },
            Renderable::Image { image: uuid },
        ))
    }

    pub fn add_text(&mut self, info: AddTextInfo) -> Entity {
        let AddTextInfo {
            x,
            y,
            z,
            scale,
            rotation,
            r,
            g,
            b,
            a,
            text,
            font_size,
        } = info;
        self.world.push((
            Position { x, y, z },
            UniformScale { scale },
            Rotation { rotation },
            Color { r, g, b },
            Opacity { opacity: a },
            Renderable::Text { text, font_size },
        ))
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.world.remove(entity);
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.key_input.key_down(key)
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.key_input.key_pressed(key)
    }

    pub fn key_up(&self, key: KeyCode) -> bool {
        self.key_input.key_up(key)
    }

    pub fn start_animation(
        &mut self,
        entity: Entity,
        name: impl ToString,
    ) -> Result<AnimationFinishChecker> {
        self.resources
            .get_mut::<AnimationStore>()
            .unwrap()
            .insert_animation_components(entity, name, &mut self.world, self.current_time)
    }
}
impl EventHandler<GameError> for AnimationEngineContext {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update timer
        let delta_time = Instant::now() - self.current_time;
        if delta_time > Duration::from_millis(35) {
            self.delta_time = Duration::from_nanos(0_016_666_666);
        } else {
            self.delta_time = delta_time;
        }
        self.current_time = self.current_time + self.delta_time;
        self.resources.insert::<Duration>(self.delta_time);
        self.resources.insert::<Instant>(self.current_time);

        // Main Update Function
        if let Some(mut update) = self.update_function.take() {
            update(self);
            self.update_function = Some(update);
        }

        // Reset Key Input
        self.key_input.reset_current_frame_input();

        // Update Animation
        self.schedule.execute(&mut self.world, &mut self.resources);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        render(ctx, &self.world, &self.resources)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        self.key_input.set_down(keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: event::KeyMods) {
        self.key_input.set_up(keycode);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            self.key_input.reset();
        }
    }
}

pub struct AnimationEngine {
    inner: AnimationEngineContext,
    ctx: Context,
    events_loop: EventLoop<()>,
}
impl AnimationEngine {
    pub fn new() -> anyhow::Result<Self> {
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };

        let cb = ContextBuilder::new("Sample Game", "Orito Itsuki")
            .window_setup(ggez::conf::WindowSetup {
                title: "rust async executor and rpg!".to_string(),
                ..Default::default()
            })
            .window_mode(ggez::conf::WindowMode {
                width: 1280.0,
                height: 720.0,
                ..Default::default()
            })
            .add_resource_path(resource_dir);

        let (mut ctx, events_loop) = cb.build().expect("Failed to create event loop");

        let mut inner = AnimationEngineContext::new();

        inner.resources.insert::<graphics::Font>(
            graphics::Font::new(&mut ctx, "/font/07LogoTypeGothic-Condense.ttf").unwrap(),
        );

        Ok(Self {
            inner,
            ctx,
            events_loop,
        })
    }

    pub fn load_animation_json(
        &mut self,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };
        let path = &resource_dir.join(path);
        self.inner
            .resources
            .get_mut::<AnimationStore>()
            .unwrap()
            .load_animation_json(name, path)
    }

    pub fn load_image(
        &mut self,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        self.inner
            .resources
            .get_mut::<ImageStore>()
            .unwrap()
            .load_image(&mut self.ctx, name, path)
    }

    pub fn get_context(&mut self) -> &mut AnimationEngineContext {
        &mut self.inner
    }

    pub fn run_with_update_func(
        mut self,
        update: impl FnMut(&mut AnimationEngineContext) -> () + 'static,
    ) -> anyhow::Result<()> {
        self.inner.update_function = Some(Box::new(update));
        event::run(self.ctx, self.events_loop, self.inner)
    }
}
