use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct ItemId(pub usize);

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ItemTarget {
    Player,
    Enemy,
}

#[derive(Deserialize)]
pub struct ItemData {
    pub id: ItemId,
    pub item_name: String,
    pub item_name_with_count: String,
    pub item_description: String,
    pub item_target: ItemTarget,
}
