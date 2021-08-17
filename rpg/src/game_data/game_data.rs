use animation_engine::*;
use std::io;

use crate::game_data::*;

pub struct GameData {
    opening_data: OpeningData,
    player_data: Vec<PlayerData>,
    item_data: Vec<ItemData>,
}
impl GameData {
    pub fn load(engine: &mut AnimationEngine) -> anyhow::Result<Self> {
        let opening_data: OpeningData = {
            let file = engine.filesystem().open("/game_data/opening-data.yml")?;
            let reader = io::BufReader::new(file);
            serde_yaml::from_reader(reader)?
        };

        let mut player_data = vec![];
        for path in engine.filesystem().read_dir("/game_data/player-data/")? {
            let file = engine.filesystem().open(path)?;
            let reader = io::BufReader::new(file);
            player_data.push(serde_yaml::from_reader(reader)?);
        }
        player_data.sort_by_cached_key(|p: &PlayerData| p.index);

        let mut item_data: Vec<ItemData> = {
            let file = engine.filesystem().open("/game_data/item-data.yml")?;
            let reader = io::BufReader::new(file);
            serde_yaml::from_reader(reader)?
        };
        item_data.sort_by_cached_key(|i: &ItemData| i.id);

        Ok(Self {
            opening_data,
            player_data,
            item_data,
        })
    }

    pub fn opening_data(&self) -> &OpeningData {
        &self.opening_data
    }

    pub fn player_data(&self) -> &Vec<PlayerData> {
        &self.player_data
    }

    pub fn item_data(&self) -> &Vec<ItemData> {
        &self.item_data
    }
}
