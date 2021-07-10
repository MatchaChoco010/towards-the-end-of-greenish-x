use ggez::event::KeyCode;
use std::collections::HashMap;

pub(crate) struct KeyInputState {
    down_hashmap: HashMap<KeyCode, bool>,
    pressed_hashmap: HashMap<KeyCode, bool>,
    up_hashmap: HashMap<KeyCode, bool>,
}
impl KeyInputState {
    pub(crate) fn new() -> Self {
        Self {
            down_hashmap: HashMap::new(),
            pressed_hashmap: HashMap::new(),
            up_hashmap: HashMap::new(),
        }
    }

    pub(crate) fn reset_current_frame_input(&mut self) {
        for (_k, flag) in self.down_hashmap.iter_mut() {
            *flag = false;
        }
        for (_k, flag) in self.up_hashmap.iter_mut() {
            *flag = false;
        }
    }

    pub(crate) fn set_down(&mut self, key: KeyCode) {
        self.down_hashmap.insert(key, true);
        self.pressed_hashmap.insert(key, true);
    }

    pub(crate) fn set_up(&mut self, key: KeyCode) {
        self.pressed_hashmap.insert(key, false);
        self.up_hashmap.insert(key, true);
    }

    pub(crate) fn reset(&mut self) {
        for (_k, flag) in self.down_hashmap.iter_mut() {
            *flag = false;
        }
        for (_k, flag) in self.pressed_hashmap.iter_mut() {
            *flag = false;
        }
        for (_k, flag) in self.up_hashmap.iter_mut() {
            *flag = false;
        }
    }

    pub(crate) fn key_down(&self, key: KeyCode) -> bool {
        if let Some(&down) = self.down_hashmap.get(&key) {
            down
        } else {
            false
        }
    }

    pub(crate) fn key_pressed(&self, key: KeyCode) -> bool {
        if let Some(&pressed) = self.pressed_hashmap.get(&key) {
            pressed
        } else {
            false
        }
    }

    pub(crate) fn key_up(&self, key: KeyCode) -> bool {
        if let Some(&up) = self.up_hashmap.get(&key) {
            up
        } else {
            false
        }
    }
}
