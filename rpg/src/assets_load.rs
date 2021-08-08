use animation_engine::*;
use log::{info, trace};
use serde::Deserialize;
use std::path::{Path, PathBuf};

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

pub fn load(engine: &mut AnimationEngine) -> anyhow::Result<()> {
    load_bgm(engine)?;

    engine.load_font(
        "/font/LogoTypeGothicCondense/07LogoTypeGothic-Condense.ttf",
        "/font/LogoTypeGothicCondense/07LogoTypeGothic-Condense.ttf",
    )?;

    Ok(())
}
