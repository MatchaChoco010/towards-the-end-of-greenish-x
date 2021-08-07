use ggez::audio::*;
use ggez::*;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

pub enum AudioPlayOption {
    Play,
    Resume,
}

pub struct AudioStore {
    sfx_hashmap: HashMap<String, Source>,
    sfx_queue: VecDeque<String>,
    bgm_hashmap: HashMap<String, Source>,
    next_bgm: Option<(String, AudioPlayOption)>,
    current_bgm: Option<String>,
    bgm_volume: f32,
}
impl AudioStore {
    pub fn new() -> Self {
        Self {
            sfx_hashmap: HashMap::new(),
            sfx_queue: VecDeque::new(),
            bgm_hashmap: HashMap::new(),
            next_bgm: None,
            current_bgm: None,
            bgm_volume: 1.0,
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
        repeat: bool,
    ) -> GameResult {
        let mut source = Source::new(ctx, path)?;
        source.set_repeat(repeat);
        self.bgm_hashmap.insert(name.to_string(), source);
        Ok(())
    }

    pub fn push_sfx_to_queue(&mut self, name: impl ToString) {
        self.sfx_queue.push_back(name.to_string());
    }

    pub fn set_bgm(&mut self, name: impl ToString, option: AudioPlayOption) {
        self.next_bgm = Some((name.to_string(), option));
    }

    pub fn get_bgm_volume(&self) -> f32 {
        self.bgm_volume
    }

    pub fn set_bgm_volume(&mut self, volume: f32) {
        self.bgm_volume = volume;
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
        if let Some((bgm_name, option)) = self.next_bgm.take() {
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
                match option {
                    AudioPlayOption::Play => {
                        source.stop(ctx)?;
                        source.play(ctx)?;
                    }
                    AudioPlayOption::Resume => {
                        source.resume();
                    }
                }
            } else {
                source.play(ctx)?;
            }

            self.current_bgm = Some(bgm_name);
        }
        // update bgm volume
        if let Some(current_bgm_name) = self.current_bgm.as_ref() {
            let source = self
                .bgm_hashmap
                .get_mut(current_bgm_name)
                .expect(&format!("No such audio: {}", current_bgm_name));
            source.set_volume(self.bgm_volume);
        }

        Ok(())
    }
}
