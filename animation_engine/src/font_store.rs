use ggez::graphics::Font;
use ggez::*;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub struct FontStore {
    font_load_queue: Vec<(String, PathBuf)>,
    font_hashmap: HashMap<String, Font>,
}
impl FontStore {
    pub fn new() -> Self {
        Self {
            font_load_queue: vec![],
            font_hashmap: HashMap::new(),
        }
    }

    pub fn add_font_to_load_queue(&mut self, name: impl ToString, path: impl AsRef<Path>) {
        self.font_load_queue
            .push((name.to_string(), path.as_ref().to_path_buf()));
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut queue = vec![];
        std::mem::swap(&mut self.font_load_queue, &mut queue);
        for (name, path) in queue {
            self.load_font(ctx, name, path)?;
        }
        Ok(())
    }

    pub fn load_font(
        &mut self,
        ctx: &mut Context,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> GameResult {
        let font = Font::new(ctx, path.as_ref())?;
        self.font_hashmap.insert(name.to_string(), font);
        Ok(())
    }

    pub fn unload_font(&mut self, name: impl ToString) -> GameResult {
        let font = self.font_hashmap.remove(&name.to_string());
        if font.is_some() {
            let _ = font;
            Ok(())
        } else {
            Err(GameError::CustomError(format!(
                "No such name font: {}",
                name.to_string()
            )))
        }
    }

    pub fn get_font(&self, name: impl ToString) -> GameResult<&Font> {
        Ok(self
            .font_hashmap
            .get(&name.to_string())
            .ok_or(GameError::CustomError(format!(
                "No such name font: {}",
                name.to_string()
            )))?)
    }
}
