use animation_engine::*;
use std::io;

use crate::game_data::OpeningData;

pub struct GameData {
    opening_data: OpeningData,
}
impl GameData {
    pub fn load(engine: &mut AnimationEngine) -> anyhow::Result<Self> {
        let opening_data: OpeningData = {
            let file = engine.filesystem().open("/game_data/opening-data.yml")?;
            let reader = io::BufReader::new(file);
            serde_yaml::from_reader(reader)?
        };

        Ok(Self { opening_data })
    }

    pub fn opening_data(&self) -> &OpeningData {
        &self.opening_data
    }
}
