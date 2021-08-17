use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemData {
    pub id: usize,
    pub item_name: String,
    pub item_name_with_count: String,
    pub item_description: String,
}
