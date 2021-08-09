pub struct LocalizeText {
    pub(crate) font_name: String,
    pub(crate) text: String,
}
impl LocalizeText {
    pub fn new(font_name: String, text: String) -> Self {
        Self { font_name, text }
    }
}

pub trait Localize {
    fn get(&self, key: &str) -> LocalizeText;
}
