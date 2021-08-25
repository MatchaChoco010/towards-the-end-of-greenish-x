use serde::Deserialize;

#[derive(Deserialize)]
pub struct RandomBranch {
    pub weight: f64,
    pub item: Box<LevelItem>,
}

#[derive(Deserialize)]
pub struct ChoiceBranch {
    pub text: String,
    pub text_lines: u8,
    pub item: Box<LevelItem>,
}

#[derive(Deserialize)]
pub struct SkillRarityWeight {
    pub weight: f64,
    pub rarity: u8,
}

#[derive(Deserialize, Clone, Copy)]
pub enum BattleTime {
    Morning,
    Afternoon,
    Night,
}

#[derive(Deserialize)]
pub enum LevelItem {
    Sequence {
        items: Vec<Box<LevelItem>>,
    },
    Random {
        branches: Vec<RandomBranch>,
    },
    Message {
        text: String,
        blue: Option<bool>,
        no_weight: Option<bool>,
    },
    Choice {
        text: String,
        branches: Vec<ChoiceBranch>,
    },
    PlayBGM {
        bgm: String,
    },
    StopBGM,
    ResumeOrPlayBGM {
        bgm: String,
    },
    ChangeToAfternoon,
    ChangeToNight,
    Battle {
        id: usize,
        bgm: String,
        time: BattleTime,
    },
    WaitOpenSkillItemList,
    GetSkill {
        skills: Vec<SkillRarityWeight>,
        count: u8,
    },
    AddItem {
        item_id: usize,
        count: usize,
    },
}

#[derive(Deserialize)]
pub struct LevelData {
    pub index: u32,
    pub item: LevelItem,
}
