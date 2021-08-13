use serde::Deserialize;

#[derive(Deserialize)]
pub struct PrologueMessage(pub String);

#[derive(Deserialize)]
pub struct PrologueIndex {
    pub index: usize,
    pub messages: Vec<PrologueMessage>,
}

#[derive(Deserialize)]
pub struct PlayerData {
    pub index: usize,
    pub image: String,
    pub shadow_image: String,
    pub opening_legendary_name: String,
    pub opening_introduction_text: String,
    pub prologue: Vec<PrologueIndex>,
}
