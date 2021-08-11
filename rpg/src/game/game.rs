use animation_engine::executor::*;
use animation_engine::*;
use log::{info, trace};

use crate::game::title;
use crate::save_data;
use crate::save_data::SaveData;

pub struct GlobalData<'a> {
    cx: &'a AnimationEngineContext,
    overlay_image: Entity,
    save_data: save_data::SaveData,
}
impl<'a> GlobalData<'a> {
    fn new(cx: &'a AnimationEngineContext) -> anyhow::Result<Self> {
        trace!("Create global data");

        trace!("Load overlay image");
        let overlay_image = cx.add_image(AddImageInfo {
            name: "/image/ui/title-overlay.png".into(),
            z: 1000,
            ..Default::default()
        });

        let save_data = SaveData::load()?;

        Ok(Self {
            cx,
            overlay_image,
            save_data,
        })
    }

    pub fn save_data(&mut self) -> &mut SaveData {
        &mut self.save_data
    }
}
impl<'a> Drop for GlobalData<'a> {
    fn drop(&mut self) {
        trace!("Drop global data");

        self.cx.delete_entity(self.overlay_image);
        self.save_data.save().expect("Failed to save data");
    }
}

pub async fn game(cx: AnimationEngineContext) {
    info!("Start game!");

    cx.change_clear_color((0, 0, 0));

    let mut global_data = GlobalData::new(&cx).expect("Failed to create global data");
    global_data.save_data().apply(&cx);

    loop {
        if let title::TitleResult::Exit = title::title(&cx, &mut global_data).await {
            info!("Exit game!");
            return;
        }
        next_frame().await;
    }
}
