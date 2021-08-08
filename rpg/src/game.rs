use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, FutureExt};
use log::{info, trace};
use std::time::Duration;

use crate::input;

pub async fn game(mut cx: AnimationEngineContext) {
    trace!("Start game!");

    cx.change_clear_color((0, 0, 0));

    spawn({
        let mut cx = cx.clone();
        async move {
            let mut bgm_volume: f32 = 1.0;
            loop {
                select! {
                    _ = input::wait_left(&cx).fuse() => bgm_volume = (bgm_volume - 0.1).max(0.0),
                    _ = input::wait_right(&cx).fuse() => bgm_volume = (bgm_volume + 0.1).min(1.2),
                }
                cx.set_bgm_volume(bgm_volume);
                cx.play_sfx("/audio/sfx/cursor.ogg");

                info!("[change bgm volume] {}", bgm_volume);

                delay(Duration::from_micros(300)).await;
            }
        }
    });

    spawn({
        let mut cx = cx.clone();
        async move {
            let mut sfx_volume: f32 = 1.0;
            loop {
                select! {
                    _ = input::wait_up(&cx).fuse() => sfx_volume = (sfx_volume + 0.1).min(1.2),
                    _ = input::wait_down(&cx).fuse() => sfx_volume = (sfx_volume - 0.1).max(0.0),
                }
                cx.set_sfx_volume(sfx_volume);
                cx.play_sfx("/audio/sfx/cursor.ogg");

                info!("[change sfx volume] {}", sfx_volume);

                delay(Duration::from_micros(300)).await;
            }
        }
    });

    spawn({
        let mut cx = cx.clone();
        async move {
            loop {
                input::wait_cancel_button(&cx).await;
                cx.play_sfx("/audio/sfx/cancel.ogg");
                next_frame().await;
            }
        }
    });

    spawn({
        let mut cx = cx.clone();
        async move {
            input::wait_sub_button(&cx).await;
            cx.play_sfx("/audio/sfx/menu.ogg");
            delay(Duration::from_secs(1)).await;
            cx.quit();
        }
    });

    let mut bgm_index = 0;
    let mut text_entity = None;
    loop {
        input::wait_select_button(&cx).await;

        if let Some(text_entity) = text_entity {
            cx.delete_entity(text_entity);
        }
        let title = match bgm_index {
            0 => "title",
            1 => "opening",
            2 => "field-0",
            3 => "field-1",
            4 => "battle-0",
            5 => "game-over",
            _ => unreachable!(),
        };
        text_entity = Some(cx.add_text(AddTextInfo {
            text: title.to_string(),
            font_name: "/font/LogoTypeGothicCondense/07LogoTypeGothic-Condense.ttf".to_string(),
            x: 50.0,
            y: 20.0,
            ..Default::default()
        }));

        match bgm_index {
            0 => cx.play_bgm("title"),
            1 => cx.play_bgm("opening"),
            2 => cx.resume_or_play_bgm("field-0"),
            3 => cx.resume_or_play_bgm("field-1"),
            4 => cx.play_bgm("battle-0"),
            5 => cx.play_bgm("game-over"),
            _ => unreachable!(),
        };
        cx.play_sfx("/audio/sfx/select.ogg");

        info!("change bgm! bgm index: {}", bgm_index);

        bgm_index = (bgm_index + 1) % 6;
        next_frame().await;
    }
}
