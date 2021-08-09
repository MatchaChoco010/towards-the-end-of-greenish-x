use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, FutureExt};
use log::{info, trace};
use std::time::Duration;

use crate::game::title;
use crate::input;

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

    spawn({
        let cx = cx.clone();
        async move {
            let mut bgm_volume: f32 = 0.3;
            cx.set_bgm_volume(bgm_volume);
            loop {
                select! {
                    _ = input::wait_left(&cx).fuse() => bgm_volume = (bgm_volume - 0.1).max(0.0),
                    _ = input::wait_right(&cx).fuse() => bgm_volume = (bgm_volume + 0.1).min(1.2),
                }
                cx.set_bgm_volume(bgm_volume);
                cx.play_sfx("/audio/sfx/cursor.ogg");

                trace!("[change bgm volume] {}", bgm_volume);

                delay(Duration::from_micros(300)).await;
            }
        }
    });

    loop {
        match title::title(&cx).await {
            title::TitleResult::StartGame => next_frame().await,
            title::TitleResult::Exit => {
                info!("Exit game!");
                cx.quit();
            }
        }
    }
}
