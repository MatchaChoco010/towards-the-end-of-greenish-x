use animation_engine::executor::*;
use animation_engine::*;
use futures::Future;
use log::{info, trace};
use rand::prelude::*;
use std::cell::{RefCell, RefMut};
use std::pin::Pin;

use crate::game::*;
use crate::game_data;
use crate::save_data;

pub struct GlobalData {
    rng: RefCell<ThreadRng>,
    cx: AnimationEngineContext,
    overlay_image: Entity,
    save_data: save_data::SaveData,
    game_data: game_data::GameData,
}
impl GlobalData {
    pub fn load(engine: &mut AnimationEngine) -> anyhow::Result<Self> {
        trace!("Create global data");

        let rng = RefCell::new(rand::thread_rng());

        let cx = engine.get_context().clone();

        trace!("Load overlay image");
        let overlay_image = cx.add_image(AddImageInfo {
            name: "/image/ui/title-overlay.png".into(),
            z: 1000,
            ..Default::default()
        });

        let save_data = save_data::SaveData::load()?;
        let game_data = game_data::GameData::load(engine)?;

        Ok(Self {
            rng,
            cx,
            overlay_image,
            save_data,
            game_data,
        })
    }

    pub fn rng(&self) -> RefMut<ThreadRng> {
        self.rng.borrow_mut()
    }

    pub fn save_data(&mut self) -> &mut save_data::SaveData {
        &mut self.save_data
    }

    pub fn game_data(&self) -> &game_data::GameData {
        &self.game_data
    }
}
impl Drop for GlobalData {
    fn drop(&mut self) {
        trace!("Drop global data");

        self.cx.delete_entity(self.overlay_image);
        self.save_data.save().expect("Failed to save data");
    }
}

async fn main(cx: AnimationEngineContext, mut global_data: GlobalData) {
    cx.change_clear_color((0, 0, 0));
    global_data.save_data().apply(&cx);

    loop {
        if let title::TitleResult::Exit = title::title(&cx, &mut global_data).await {
            break;
        }
        let opening::PlayerIndex(index) = opening::opening(&cx, &mut global_data).await;
        // let index = 0;
        explore::explore(&cx, &mut global_data, index).await;
        crate::input::wait_select_button(&cx).await;
        next_frame().await;
    }
}

pub fn game(
    global_data: GlobalData,
) -> impl FnOnce(AnimationEngineContext) -> Pin<Box<dyn Future<Output = ()> + 'static>> {
    info!("Start game!");
    move |cx: AnimationEngineContext| {
        Box::pin(async move {
            main(cx, global_data).await;
            info!("Exit game!");
        })
    }
}
