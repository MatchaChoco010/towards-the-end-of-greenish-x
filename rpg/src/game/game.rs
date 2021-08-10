use animation_engine::executor::*;
use animation_engine::*;
use log::{info, trace};

use crate::game::title;

struct OverlayImage<'a> {
    cx: &'a AnimationEngineContext,
    image: Entity,
}
impl<'a> OverlayImage<'a> {
    fn new(cx: &'a AnimationEngineContext) -> Self {
        trace!("load overlay image.");

        let image = cx.add_image(AddImageInfo {
            name: "/image/ui/title-overlay.png".into(),
            z: 1000,
            ..Default::default()
        });
        Self { cx, image }
    }
}
impl<'a> Drop for OverlayImage<'a> {
    fn drop(&mut self) {
        trace!("drop overlay image.");

        self.cx.delete_entity(self.image);
    }
}

pub async fn game(cx: AnimationEngineContext) {
    info!("Start game!");

    cx.change_clear_color((0, 0, 0));
    let _overlay_image = OverlayImage::new(&cx);

    loop {
        if let title::TitleResult::Exit = title::title(&cx).await {
            info!("Exit game!");
            return;
        }
        next_frame().await;
    }
}
