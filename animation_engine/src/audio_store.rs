use ggez::audio::*;
use ggez::*;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

pub struct AudioStore {
    sfx_hashmap: HashMap<String, Source>,
    sfx_queue: VecDeque<String>,
    bgm_hashmap: HashMap<String, Source>,
    next_bgm: Option<String>,
    current_bgm: Option<String>,
}
impl AudioStore {
    pub fn new() -> Self {
        Self {
            sfx_hashmap: HashMap::new(),
            sfx_queue: VecDeque::new(),
            bgm_hashmap: HashMap::new(),
            next_bgm: None,
            current_bgm: None,
        }
    }

    pub fn load_sfx(
        &mut self,
        ctx: &mut Context,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> GameResult {
        let source = Source::new(ctx, path)?;
        self.sfx_hashmap.insert(name.to_string(), source);
        Ok(())
    }

    pub fn load_bgm(
        &mut self,
        ctx: &mut Context,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> GameResult {
        let mut source = Source::new(ctx, path)?;
        source.set_repeat(true);
        self.bgm_hashmap.insert(name.to_string(), source);
        Ok(())
    }

    pub fn push_sfx_to_queue(&mut self, name: impl ToString) {
        self.sfx_queue.push_back(name.to_string());
    }

    pub fn set_bgm(&mut self, name: impl ToString) {
        self.next_bgm = Some(name.to_string());
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        // sfx
        for sfx in self.sfx_queue.drain(0..) {
            let source = self
                .sfx_hashmap
                .get_mut(&sfx)
                .expect(&format!("No such audio: {}", sfx));
            source.play_detached(ctx)?;
        }

        // bgm
        if let Some(bgm_name) = self.next_bgm.take() {
            if let Some(current_bgm_name) = self.current_bgm.take() {
                let source = self
                    .bgm_hashmap
                    .get(&current_bgm_name)
                    .expect(&format!("No such audio: {}", current_bgm_name));
                source.pause();
            }

            let source = self
                .bgm_hashmap
                .get_mut(&bgm_name)
                .expect(&format!("No such audio: {}", bgm_name));
            if source.paused() {
                source.resume();
            } else {
                source.play(ctx)?;
            }
            self.current_bgm = Some(bgm_name);
        }

        Ok(())
    }
}
