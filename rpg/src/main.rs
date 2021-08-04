use animation_engine::executor::*;
use animation_engine::*;
use chrono;
use fern;
use log::{info, trace};
use path_slash::PathExt;
use std::env;
use std::fs;
use std::path;

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

fn load_bgm(engine: &mut AnimationEngine) -> anyhow::Result<()> {
    trace!("Start loading bgm...");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        trace!("\tin manifest dir");

        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        trace!("\tin not manifest dir");

        path::PathBuf::from("./resources")
    };

    for item in fs::read_dir(resource_dir.join("audio").join("bgm"))? {
        if let Ok(item) = item {
            if item.file_type()?.is_file() {
                let item_path = item.path();
                let item_path = item_path.strip_prefix(&resource_dir)?;
                let filepath = path::Path::new("/").join(item_path);
                let name = item_path.to_slash_lossy();

                info!(
                    "[load bgm] key: {}, path: {}",
                    name,
                    filepath.to_string_lossy()
                );

                engine.load_bgm(name, filepath)?;
            }
        }
    }

    trace!("Finish loading bgm!");

    Ok(())
}

async fn game(mut cx: AnimationEngineContext) {
    trace!("Start game!");

    let mut bgm_index = 0;
    loop {
        cx.wait_key_down(KeyCode::Z).await;
        match bgm_index {
            0 => cx.play_bgm("audio/bgm/field-0.ogg"),
            1 => cx.play_bgm("audio/bgm/field-1.ogg"),
            2 => cx.play_bgm("audio/bgm/battle-0.ogg"),
            3 => cx.play_bgm("audio/bgm/game-over.ogg"),
            _ => unreachable!(),
        }

        info!("change bgm! bgm index: {}", bgm_index);

        bgm_index = (bgm_index + 1) % 4;
        next_frame().await;
    }
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let mut engine = AnimationEngine::new("Towards The End of Greenish-X")?;
    load_bgm(&mut engine)?;
    engine.run_with_async_func(game)
}
