use crate::animation_components::*;
use crate::audio_store::*;
use crate::image_store::*;
use crate::key_input_state::*;
use crate::render::*;
use anyhow::Result;
#[cfg(feature = "async-feature")]
use executor::*;
use ggez::event::{self, EventHandler, EventLoop, KeyCode};
use ggez::*;
use legion::*;
use std::cell::{Ref, RefCell, RefMut};
use std::env;
use std::future::Future;
use std::path::{self, Path};
use std::rc::Rc;
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

#[derive(Clone)]
pub struct AnimationEngineContext(Rc<RefCell<AnimationEngineInner>>);
impl AnimationEngineContext {
    fn get_mut(&self) -> RefMut<AnimationEngineInner> {
        self.0.borrow_mut()
    }

    fn get(&self) -> Ref<AnimationEngineInner> {
        self.0.borrow()
    }
}
pub struct AnimationEngineInner {
    current_time: Instant,
    delta_time: Duration,
    world: World,
    resources: Resources,
    schedule: Schedule,
    update_function: Option<Box<dyn FnMut(&mut AnimationEngineContext) -> ()>>,
    key_input: KeyInputState,
    quit_flag: bool,
}
impl AnimationEngineContext {
    fn new() -> Self {
        let mut schedule = Schedule::builder();
        add_animation_system(&mut schedule);
        let schedule = schedule.build();

        let mut resources = Resources::default();
        resources.insert::<AnimationStore>(AnimationStore::new());
        resources.insert::<ImageStore>(ImageStore::new());
        resources.insert::<AudioStore>(AudioStore::new());

        Self(Rc::new(RefCell::new(AnimationEngineInner {
            current_time: Instant::now(),
            delta_time: Duration::new(0, 0),
            world: World::default(),
            resources,
            schedule,
            update_function: None,
            key_input: KeyInputState::new(),
            quit_flag: false,
        })))
    }

    pub fn quit(&mut self) {
        self.get_mut().quit_flag = true;
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
        self.get_mut().world.push((
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
            .get_mut()
            .resources
            .get::<ImageStore>()
            .unwrap()
            .get_image_uuid_from_name(name)
            .expect("Failed to get image");
        self.get_mut().world.push((
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
        self.get_mut().world.push((
            Position { x, y, z },
            UniformScale { scale },
            Rotation { rotation },
            Color { r, g, b },
            Opacity { opacity: a },
            Renderable::Text { text, font_size },
        ))
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.get_mut().world.remove(entity);
    }

    pub fn set_width(&mut self, entity: Entity, width: f32) -> anyhow::Result<()> {
        match self
            .get_mut()
            .world
            .entry_mut(entity)?
            .get_component_mut::<Renderable>()
            .expect(&format!("Entity {:?} has no renderable component", entity))
        {
            Renderable::Rect { width: w, .. } => *w = width,
            _ => (),
        }
        Ok(())
    }

    pub fn set_position(&mut self, entity: Entity, x: f32, y: f32, z: u32) -> anyhow::Result<()> {
        let mut this = self.get_mut();
        let mut entry = this.world.entry_mut(entity)?;
        let pos = entry
            .get_component_mut::<Position>()
            .expect(&format!("Entity {:?} has no position component", entity));
        pos.x = x;
        pos.y = y;
        pos.z = z;
        Ok(())
    }

    pub fn play_bgm(&mut self, name: impl ToString) {
        self.get_mut()
            .resources
            .get_mut::<AudioStore>()
            .unwrap()
            .set_bgm(name);
    }

    pub fn play_sfx(&mut self, name: impl ToString) {
        self.get_mut()
            .resources
            .get_mut::<AudioStore>()
            .unwrap()
            .push_sfx_to_queue(name);
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.get().key_input.key_down(key)
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.get().key_input.key_pressed(key)
    }

    pub fn key_up(&self, key: KeyCode) -> bool {
        self.get().key_input.key_up(key)
    }

    pub async fn wait_key_down(&self, key: KeyCode) {
        loop {
            if self.get().key_input.key_down(key) {
                break;
            }
            next_frame().await;
        }
    }

    pub async fn wait_key_pressed(&self, key: KeyCode) {
        loop {
            if self.get().key_input.key_pressed(key) {
                break;
            }
            next_frame().await;
        }
    }

    pub async fn wait_key_up(&self, key: KeyCode) {
        loop {
            if self.get().key_input.key_up(key) {
                break;
            }
            next_frame().await;
        }
    }

    pub fn start_animation(
        &mut self,
        entity: Entity,
        name: impl ToString,
    ) -> Result<AnimationFinishChecker> {
        let this = &mut *self.get_mut();
        let anim_store = this.resources.get_mut::<AnimationStore>().unwrap();
        anim_store.insert_animation_components(entity, name, &mut this.world, this.current_time)
    }

    #[cfg(feature = "async-feature")]
    pub async fn play_animation(&mut self, entity: Entity, name: impl ToString) -> Result<()> {
        let mut checker = self.start_animation(entity, name)?;
        loop {
            if checker.is_finished() {
                break;
            }
            next_frame().await;
        }
        Ok(())
    }
}
impl EventHandler<GameError> for AnimationEngineContext {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update timer
        let delta_time = Instant::now() - self.get().current_time;
        if delta_time > Duration::from_millis(35) {
            self.get_mut().delta_time = Duration::from_nanos(0_016_666_666);
        } else {
            self.get_mut().delta_time = delta_time;
        }
        let delta_time = self.get().delta_time;
        self.get_mut().current_time += delta_time;
        let current_time = self.get().current_time;
        self.get_mut().resources.insert::<Duration>(delta_time);
        self.get_mut().resources.insert::<Instant>(current_time);

        // Main Update Function
        let update = self.get_mut().update_function.take();
        if let Some(mut update) = update {
            update(self);
            self.get_mut().update_function = Some(update);
        }

        // Reset Key Input
        self.get_mut().key_input.reset_current_frame_input();

        // Update Animation
        {
            let this = &mut *self.get_mut();
            let resources = &mut this.resources;
            let world = &mut this.world;
            let schedule = &mut this.schedule;
            schedule.execute(world, resources);
        }

        // Play audio
        self.get_mut()
            .resources
            .get_mut::<AudioStore>()
            .unwrap()
            .update(ctx)?;

        // Quit game if quit flag is on
        if self.get().quit_flag {
            ggez::event::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        render(ctx, &self.get().world, &self.get().resources)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        self.get_mut().key_input.set_down(keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: event::KeyMods) {
        self.get_mut().key_input.set_up(keycode);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            self.get_mut().key_input.reset();
        }
    }
}

pub struct AnimationEngine {
    inner: AnimationEngineContext,
    ctx: Context,
    events_loop: EventLoop<()>,
}
impl AnimationEngine {
    pub fn new(title: impl ToString) -> anyhow::Result<Self> {
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };

        let cb = ContextBuilder::new("Sample Game", "Orito Itsuki")
            .window_setup(ggez::conf::WindowSetup {
                title: title.to_string(),
                icon: "/icon.png".to_string(),
                ..Default::default()
            })
            .window_mode(ggez::conf::WindowMode {
                width: 1280.0,
                height: 720.0,
                ..Default::default()
            })
            .add_resource_path(resource_dir);

        let (mut ctx, events_loop) = cb.build().expect("Failed to create event loop");

        let inner = AnimationEngineContext::new();

        inner.get_mut().resources.insert::<graphics::Font>(
            graphics::Font::new(&mut ctx, "/font/07LogoTypeGothic-Condense.ttf").unwrap(),
        );

        Ok(Self {
            inner,
            ctx,
            events_loop,
        })
    }

    pub fn load_animation_yaml(
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
            .get_mut()
            .resources
            .get_mut::<AnimationStore>()
            .unwrap()
            .load_animation_yaml(name, path)
    }

    pub fn load_image(
        &mut self,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        self.inner
            .get_mut()
            .resources
            .get_mut::<ImageStore>()
            .unwrap()
            .load_image(&mut self.ctx, name, path)
    }

    pub fn load_sfx(&mut self, name: impl ToString, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.inner
            .get_mut()
            .resources
            .get_mut::<AudioStore>()
            .unwrap()
            .load_sfx(&mut self.ctx, name, path)?;
        Ok(())
    }

    pub fn load_bgm(&mut self, name: impl ToString, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.inner
            .get_mut()
            .resources
            .get_mut::<AudioStore>()
            .unwrap()
            .load_bgm(&mut self.ctx, name, path)?;
        Ok(())
    }

    pub fn get_context(&mut self) -> &mut AnimationEngineContext {
        &mut self.inner
    }

    pub fn run_with_update_func(
        self,
        update: impl FnMut(&mut AnimationEngineContext) -> () + 'static,
    ) -> anyhow::Result<()> {
        self.inner.get_mut().update_function = Some(Box::new(update));
        event::run(self.ctx, self.events_loop, self.inner)
    }

    #[cfg(feature = "async-feature")]
    pub fn run_with_async_func<F, Fut>(self, async_fn: F) -> anyhow::Result<()>
    where
        F: FnOnce(AnimationEngineContext) -> Fut + 'static,
        Fut: Future<Output = ()>,
    {
        activate_thread_local_executor();
        spawn({
            let inner = self.inner.clone();
            async move {
                async_fn(inner).await;
            }
        });
        self.inner.get_mut().update_function = Some(Box::new({
            let mut inner = self.inner.clone();
            move |cx| {
                let delta_time = cx.get().delta_time;
                match step(delta_time) {
                    StepState::RemainTasks => (),
                    StepState::Completed => inner.quit(),
                }
            }
        }));
        event::run(self.ctx, self.events_loop, self.inner)
    }
}
