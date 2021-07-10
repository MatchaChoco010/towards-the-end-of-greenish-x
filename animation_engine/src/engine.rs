// use crate::animation::*;
use crate::animation_components::*;
// use crate::components::*;
use crate::key_input_state::KeyInputState;
use crate::render::*;
use anyhow::Result;
use ggez::event::{self, EventHandler, EventLoop, KeyCode};
use ggez::*;
use legion::*;
use std::env;
use std::path;
use std::path::Path;
use std::time::{Duration, Instant};

// pub use crate::animation::AnimationFinishChecker;
pub use crate::animation_components::AnimationFinishChecker;

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

    pub fn add_rect(&mut self, x: f32, y: f32, z: u32, width: f32, height: f32) -> Entity {
        self.world.push((
            Position { x, y, z },
            Renderable::Rect { width, height },
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
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

    pub fn load_animation_json(&mut self, name: impl ToString, path: &Path) -> anyhow::Result<()> {
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };
        let path = &resource_dir.join(path);
        self.resources
            .get_mut::<AnimationStore>()
            .unwrap()
            .load_animation_json(name, path)
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
        render(ctx, &self.world)
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

        let cb = ContextBuilder::new("Sample Game", "Orito Itsuki").add_resource_path(resource_dir);

        let (ctx, events_loop) = cb.build().expect("Failed to create event loop");

        Ok(Self {
            inner: AnimationEngineContext::new(),
            ctx,
            events_loop,
        })
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
