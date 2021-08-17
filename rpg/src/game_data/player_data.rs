use serde::Deserialize;

#[derive(Deserialize)]
pub struct PrologueMessage(pub String);

#[derive(Deserialize)]
pub struct PrologueIndex {
    pub index: usize,
    pub messages: Vec<PrologueMessage>,
}

#[derive(Deserialize)]
pub struct SkillData {
    pub id: usize,
    pub skill_type: usize,
    pub reality: u8,
    pub skill_name: String,
    pub skill_name_with_level: String,
    pub skill_description: String,
}

#[derive(Deserialize)]
pub struct PlayerData {
    pub index: usize,
    pub image: String,
    pub shadow_image: String,
    pub opening_legendary_name: String,
    pub opening_introduction_text: String,
    pub prologue: Vec<PrologueIndex>,
    pub skills: Vec<SkillData>,
}
