use ggez::event::{Axis, Button};
use std::collections::HashMap;

pub(crate) struct GamepadInputState {
    down_hashmap: HashMap<Button, bool>,
    pressed_hashmap: HashMap<Button, bool>,
    up_hashmap: HashMap<Button, bool>,
    axis_hashmap: HashMap<Axis, f32>,
}
impl GamepadInputState {
    pub(crate) fn new() -> Self {
        Self {
            down_hashmap: HashMap::new(),
            pressed_hashmap: HashMap::new(),
            up_hashmap: HashMap::new(),
            axis_hashmap: HashMap::new(),
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

    pub(crate) fn set_down(&mut self, button: Button) {
        self.down_hashmap.insert(button, true);
        self.pressed_hashmap.insert(button, true);
    }

    pub(crate) fn set_up(&mut self, button: Button) {
        self.pressed_hashmap.insert(button, false);
        self.up_hashmap.insert(button, true);
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
        for (_k, value) in self.axis_hashmap.iter_mut() {
            *value = 0.0;
        }
    }

    pub(crate) fn button_down(&self, button: Button) -> bool {
        if let Some(&down) = self.down_hashmap.get(&button) {
            down
        } else {
            false
        }
    }

    pub(crate) fn button_pressed(&self, button: Button) -> bool {
        if let Some(&pressed) = self.pressed_hashmap.get(&button) {
            pressed
        } else {
            false
        }
    }

    pub(crate) fn button_up(&self, button: Button) -> bool {
        if let Some(&up) = self.up_hashmap.get(&button) {
            up
        } else {
            false
        }
    }

    pub(crate) fn set_axis(&mut self, axis: Axis, value: f32) {
        self.axis_hashmap.insert(axis, value);
    }

    pub(crate) fn axis(&self, axis: &Axis) -> f32 {
        if let Some(&value) = self.axis_hashmap.get(axis) {
            value
        } else {
            0.0
        }
    }
}
