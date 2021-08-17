use animation_engine::*;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::thread_local;

#[derive(Deserialize)]
struct LocalizeInfo {
    #[serde(alias = "font-file")]
    font_name: String,
    texts: HashMap<String, String>,
}

pub struct LocalizeTexts {
    current_index: usize,
    infos: Vec<LocalizeInfo>,
}
impl LocalizeTexts {
    fn new() -> anyhow::Result<Self> {
        let mut infos = vec![];

        let exe_path = env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap();
        let i18n_dir = exe_dir.join("i18n");

        let file = fs::File::open(&i18n_dir.join("jp-original.yml"))?;
        let reader = io::BufReader::new(file);
        let jp_info: LocalizeInfo = serde_yaml::from_reader(reader)?;
        infos.push(jp_info);

        for entry in fs::read_dir(&i18n_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                if filename.to_string_lossy() == "jp-original.yml" {
                    continue;
                }
            }
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let info: LocalizeInfo = serde_yaml::from_reader(reader)?;
            infos.push(info);
        }

        Ok(Self {
            current_index: 0,
            infos,
        })
    }

    fn init(&self, cx: &AnimationEngineContext) {
        cx.load_font(&self.infos[0].font_name, &self.infos[0].font_name);
    }

    fn get(&self, key: &str) -> LocalizeText {
        let info = &self.infos[self.current_index];
        let font_name = info.font_name.to_owned();
        let text = info
            .texts
            .get(key)
            .expect(&format!("no text data in localization file: {}", key))
            .to_owned();
        LocalizeText::new(font_name, text)
    }

    fn change_language(&mut self, cx: &AnimationEngineContext, index: usize) {
        if index >= self.infos.len() {
            panic!("out of range");
        }
        cx.unload_font(&self.infos[self.current_index].font_name)
            .unwrap();
        self.current_index = index;
        cx.load_font(
            &self.infos[self.current_index].font_name,
            &self.infos[self.current_index].font_name,
        );
    }

    fn len(&self) -> usize {
        self.infos.len()
    }
}

thread_local! {
    pub static LOCALIZE_TEXTS: RefCell<LocalizeTexts> = {
        let localize_texts = LocalizeTexts::new().expect("Failed to load i18n");
        RefCell::new(localize_texts)
    }
}

struct LocalizeTraitObject;
impl Localize for LocalizeTraitObject {
    fn get(&self, key: &str) -> LocalizeText {
        LOCALIZE_TEXTS.with(|texts| texts.borrow().get(key))
    }
}

pub fn set_localize(engine: &mut AnimationEngine) {
    engine.set_localize(Box::new(LocalizeTraitObject));
    LOCALIZE_TEXTS.with(|texts| texts.borrow().init(engine.get_context()))
}

pub fn len() -> usize {
    LOCALIZE_TEXTS.with(|texts| texts.borrow().len())
}

pub fn change_language(cx: &AnimationEngineContext, index: usize) {
    LOCALIZE_TEXTS.with(|texts| texts.borrow_mut().change_language(cx, index))
}
