use animation_engine::AnimationEngineContext;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;

use crate::localization;

const MAX_VOLUME: u8 = 15;

#[derive(Deserialize, Serialize)]
struct SaveDataContent {
    version: u8,
    bgm_volume: u8,
    sfx_volume: u8,
    language: usize,
}
impl SaveDataContent {
    fn new() -> Self {
        Self {
            version: 0,
            bgm_volume: 7,
            sfx_volume: 7,
            language: 0,
        }
    }
}
pub struct SaveData {
    file: fs::File,
    data: SaveDataContent,
}
impl SaveData {
    pub fn load() -> anyhow::Result<Self> {
        info!("Load save_data.");

        let exe_path = env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap();
        let save_data_path = exe_dir.join("save_data");
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(save_data_path)?;
        let reader = io::BufReader::new(&file);
        if let Ok(data) = rmp_serde::from_read::<_, SaveDataContent>(reader) {
            trace!("Load save_data file.");
            Ok(Self { file, data })
        } else {
            trace!("Create save_data file.");
            Ok(Self {
                file,
                data: SaveDataContent::new(),
            })
        }
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        info!("Save save_data.");
        let mut writer = io::BufWriter::new(&mut self.file);
        rmp_serde::encode::write(&mut writer, &self.data)?;
        Ok(())
    }

    pub fn bgm_volume(&self) -> u8 {
        self.data.bgm_volume
    }

    pub fn bgm_volume_up(&mut self) -> anyhow::Result<()> {
        trace!("Bgm volume up.");
        self.data.bgm_volume = (self.data.bgm_volume + 1).min(MAX_VOLUME);
        self.save()
    }

    pub fn bgm_volume_down(&mut self) -> anyhow::Result<()> {
        trace!("Bgm volume down.");
        if self.data.bgm_volume > 0 {
            self.data.bgm_volume = self.data.bgm_volume - 1;
        }
        self.save()
    }

    pub fn sfx_volume(&self) -> u8 {
        self.data.sfx_volume
    }

    pub fn sfx_volume_up(&mut self) -> anyhow::Result<()> {
        trace!("Sfx volume up.");
        self.data.sfx_volume = (self.data.sfx_volume + 1).min(MAX_VOLUME);
        self.save()
    }

    pub fn sfx_volume_down(&mut self) -> anyhow::Result<()> {
        trace!("Sfx volume down.");
        if self.data.sfx_volume > 0 {
            self.data.sfx_volume = self.data.sfx_volume - 1;
        }
        self.save()
    }

    pub fn language(&self) -> usize {
        self.data.language
    }

    pub fn set_language(&mut self, index: usize) -> anyhow::Result<()> {
        trace!("Change and save language.");
        self.data.language = index;
        self.save()
    }

    pub fn apply(&self, cx: &AnimationEngineContext) {
        cx.set_bgm_volume(self.bgm_volume() as f32 * 0.1);
        cx.set_sfx_volume(self.sfx_volume() as f32 * 0.1);
        localization::change_language(cx, self.language() as usize);
    }
}
