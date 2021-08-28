use serde::Deserialize;

#[derive(Deserialize)]
pub struct PrologueMessage(pub String);

#[derive(Deserialize)]
pub struct PrologueIndex {
    pub index: usize,
    pub messages: Vec<PrologueMessage>,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct SkillId(pub usize);

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SkillCost {
    Cost(u32),
    Infinity,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SkillTarget {
    Player,
    Enemy,
}

#[derive(Deserialize)]
pub struct SkillData {
    pub id: SkillId,
    pub skill_type: usize,
    pub rarity: u8,
    pub rarity_weight: f64,
    pub skill_name: String,
    pub skill_name_with_level: String,
    pub skill_description: String,
    pub get_skill_confirm_message: String,
    pub skill_cost: SkillCost,
    pub skill_target: SkillTarget,
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
