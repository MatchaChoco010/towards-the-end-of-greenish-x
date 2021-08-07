use animation_engine::executor::*;
use animation_engine::*;
use chrono;
use fern;
use futures::{select, FutureExt};
use log::{info, trace};
use serde::Deserialize;
use std::path::{Path, PathBuf};

fn init_logger() {
    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .level_for("rpg", log::LevelFilter::Trace)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("game.log").unwrap());

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .level_for("rpg", log::LevelFilter::Info)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}

#[derive(Deserialize)]
struct Bgm {
    name: String,
    path: PathBuf,
    #[serde(rename = "loop")]
    is_loop: bool,
}

#[derive(Deserialize)]
struct BgmList(Vec<Bgm>);

fn load_bgm(engine: &mut AnimationEngine) -> anyhow::Result<()> {
    trace!("Start loading bgm...");

    let file = engine.filesystem().open("/audio/bgm/bgm-list.yml")?;
    let reader = std::io::BufReader::new(file);
    let list: BgmList = serde_yaml::from_reader(reader)?;
    for bgm in list.0 {
        let filepath = Path::new("/audio/bgm/").join(bgm.path);

        info!(
            "[load bgm] name: {}, loop: {}, path: {}",
            bgm.name,
            bgm.is_loop,
            filepath.clone().to_string_lossy()
        );

        engine.load_bgm(bgm.name, filepath, bgm.is_loop)?;
    }

    trace!("Finish loading bgm!");

    Ok(())
}

async fn game(mut cx: AnimationEngineContext) {
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

fn main() -> anyhow::Result<()> {
    init_logger();

    let mut engine = AnimationEngine::new("Towards The End of Greenish-X")?;
    load_bgm(&mut engine)?;
    engine.load_font(
        "07LogoTypeGothic-Condense",
        "/font/LogoTypeGothicCondense/07LogoTypeGothic-Condense.ttf",
    )?;
    engine.run_with_async_func(game)
}
