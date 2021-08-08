use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, FutureExt};
use log::{info, trace};

pub async fn game(mut cx: AnimationEngineContext) {
    trace!("Start game!");

    cx.change_clear_color((0, 0, 0));

    spawn({
        let mut cx = cx.clone();
        async move {
            let mut bgm_volume: f32 = 1.0;
            loop {
                select! {
                    _ = cx.wait_key_down(KeyCode::Left).fuse() =>
                        bgm_volume = (bgm_volume - 0.1).max(0.0),
                    _ = cx.wait_key_down(KeyCode::Right).fuse() =>
                        bgm_volume = (bgm_volume + 0.1).min(1.2),
                }
                cx.set_bgm_volume(bgm_volume);

                info!("[change bgm volume] {}", bgm_volume);

                next_frame().await;
            }
        }
    });

    let mut bgm_index = 0;
    loop {
        cx.wait_key_down(KeyCode::Z).await;
        match bgm_index {
            0 => cx.play_bgm("title"),
            1 => cx.play_bgm("opening"),
            2 => cx.resume_or_play_bgm("field-0"),
            3 => cx.resume_or_play_bgm("field-1"),
            4 => cx.play_bgm("battle-0"),
            5 => cx.play_bgm("game-over"),
            _ => unreachable!(),
        }

        info!("change bgm! bgm index: {}", bgm_index);

        bgm_index = (bgm_index + 1) % 6;
        next_frame().await;
    }
}
