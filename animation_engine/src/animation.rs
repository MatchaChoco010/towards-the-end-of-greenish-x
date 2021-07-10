use anyhow::Result;
use legion::systems::Builder;
use legion::systems::CommandBuffer;
use legion::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use thiserror::Error;
use uuid::Uuid;

use crate::components::Position;

// 共通のシリアライズデータ
#[derive(Deserialize, Clone, Copy, Debug)]
struct Float32Keyframe {
    frame: u32,
    value: f32,
}

#[derive(Clone, Copy)]
struct Float32KeyRange {
    start: Float32Keyframe,
    end: Float32Keyframe,
}
impl Float32KeyRange {
    fn get_value(&self, duration: Duration, fps: f32) -> f32 {
        if self.end.frame == u32::MAX {
            return self.start.value;
        }

        let range_len = self.end.frame as f32 / fps - self.start.frame as f32 / fps;
        let range_duration = duration.as_secs_f32() - self.start.frame as f32 / fps;
        let t = range_duration / range_len;
        self.start.value + t * (self.end.value - self.start.value)
    }
}

#[derive(Deserialize, Debug)]
struct Float32AnimationData {
    keys: Vec<Float32Keyframe>,
}
impl Float32AnimationData {
    fn get_range(&self, frame: u32) -> Float32KeyRange {
        let first_frame = Float32Keyframe {
            frame: 0,
            value: self.keys[0].value,
        };
        let last_frame = Float32Keyframe {
            frame: u32::MAX,
            value: self.keys.last().unwrap().value,
        };
        [first_frame]
            .iter()
            .chain(self.keys.iter())
            .zip(self.keys.iter().chain([last_frame].iter()))
            .map(|(&s, &e)| Float32KeyRange { start: s, end: e })
            .find(|range| range.start.frame <= frame && frame < range.end.frame)
            .unwrap()
    }
}

// #[derive(Clone, Copy)]
// pub(crate) struct Position {
//     pub x: f32,
//     pub y: f32,
//     pub z: u32,
// }

// シリアライズに使うやつ
#[derive(Deserialize, Debug, Clone, Copy)]
enum AnimationPropertyType {
    #[allow(non_camel_case_types)]
    Position_x,
    #[allow(non_camel_case_types)]
    Position_y,
}
#[derive(Deserialize, Debug)]
struct EntityAnimationSerializeData {
    len: u32,
    fps: f32,
    data: Vec<(AnimationPropertyType, Float32AnimationData)>,
}

// メモリ上のStore
struct EntityAnimationData {
    len: u32,
    fps: f32,
    data: Vec<Uuid>,
}
pub struct AnimationStore {
    float_32_animations: HashMap<Uuid, (AnimationPropertyType, Float32AnimationData)>,
    entity_animations: HashMap<String, EntityAnimationData>,
}
impl AnimationStore {
    pub fn new() -> Self {
        Self {
            float_32_animations: HashMap::new(),
            entity_animations: HashMap::new(),
        }
    }

    pub fn load_animation_json(&mut self, name: impl ToString, path: &Path) -> anyhow::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let anim_data: EntityAnimationSerializeData = serde_json::from_reader(reader)?;

        let mut data = vec![];
        for d in anim_data.data {
            let uuid = uuid::Uuid::new_v4();
            self.float_32_animations.insert(uuid, d);
            data.push(uuid);
        }

        self.entity_animations.insert(
            name.to_string(),
            EntityAnimationData {
                len: anim_data.len,
                fps: anim_data.fps,
                data,
            },
        );

        Ok(())
    }

    pub fn insert_animation_components(
        &self,
        entity: Entity,
        name: impl ToString,
        world: &mut World,
        start_time: Instant,
    ) -> Result<AnimationFinishChecker> {
        if let Some(mut entry) = world.entry(entity) {
            if let Some(EntityAnimationData { len, fps, data }) =
                self.entity_animations.get(&name.to_string())
            {
                data.iter()
                    .map(|uuid| (uuid, self.float_32_animations.get(uuid).unwrap()))
                    .for_each(|(&uuid, (ty, _))| match ty {
                        AnimationPropertyType::Position_x => {
                            entry.add_component(AnimationPropertyComponent_Position_x { uuid })
                        }
                        AnimationPropertyType::Position_y => {
                            entry.add_component(AnimationPropertyComponent_Position_y { uuid })
                        }
                    });
                let (checker, component) = EntityAnimationComponent::new(
                    *len,
                    *fps,
                    start_time,
                    data.iter()
                        .map(|uuid| self.float_32_animations.get(uuid).unwrap().0)
                        .collect::<Vec<_>>(),
                );
                entry.add_component(component);
                Ok(checker)
            } else {
                Err(AnimationError::NotRegisteredSuchAnimation(name.to_string()))?
            }
        } else {
            Err(AnimationError::NotExistsSuchEntity(entity))?
        }
    }
}

#[derive(Error, Debug)]
pub enum AnimationError {
    #[error("the entity `{0:?}` is not exists in the world")]
    NotExistsSuchEntity(Entity),
    #[error("the animation `{0}` is not registered yet")]
    NotRegisteredSuchAnimation(String),
}

struct AnimationFinishSignal;
pub struct AnimationFinishChecker {
    is_finished: bool,
    receiver: Receiver<AnimationFinishSignal>,
}
impl AnimationFinishChecker {
    fn new(receiver: Receiver<AnimationFinishSignal>) -> Self {
        Self {
            is_finished: false,
            receiver,
        }
    }

    pub fn is_finished(&mut self) -> bool {
        if self.is_finished {
            true
        } else {
            if let Ok(_) = self.receiver.try_recv() {
                self.is_finished = true;
                true
            } else {
                false
            }
        }
    }
    pub fn is_playing(&mut self) -> bool {
        !self.is_finished()
    }
}

// 実行時に使うComponent
enum EntityAnimationUpdateStatus {
    Playing,
    JustFinish,
    Finished,
}
struct EntityAnimationComponent {
    len: u32,
    fps: f32,
    current_frame: u32,
    start_time: Instant,
    current_time: Instant,
    animation_types: Vec<AnimationPropertyType>,
    is_finished: bool,
    finish_sender: Arc<Mutex<Sender<AnimationFinishSignal>>>,
}
impl EntityAnimationComponent {
    fn new(
        len: u32,
        fps: f32,
        start_time: Instant,
        animation_types: Vec<AnimationPropertyType>,
    ) -> (AnimationFinishChecker, Self) {
        let (sender, receiver) = channel();
        (
            AnimationFinishChecker::new(receiver),
            EntityAnimationComponent {
                len,
                fps,
                current_frame: 0,
                start_time,
                current_time: start_time,
                animation_types,
                is_finished: false,
                finish_sender: Arc::new(Mutex::new(sender)),
            },
        )
    }

    fn update(&mut self, delta_time: Duration) -> EntityAnimationUpdateStatus {
        self.current_time += delta_time;
        let duration = self.current_time - self.start_time;
        let frame = (duration.as_secs_f32() * self.fps).floor() as u32;
        let frame = frame.min(self.len);
        self.current_frame = frame;

        if frame >= self.len {
            if !self.is_finished {
                self.is_finished = true;
                self.finish_sender
                    .lock()
                    .unwrap()
                    .send(AnimationFinishSignal)
                    .unwrap();
                EntityAnimationUpdateStatus::JustFinish
            } else {
                EntityAnimationUpdateStatus::Finished
            }
        } else {
            EntityAnimationUpdateStatus::Playing
        }
    }
}
#[allow(non_camel_case_types)]
struct AnimationPropertyComponent_Position_x {
    uuid: Uuid,
}
impl AnimationPropertyComponent_Position_x {
    fn update(
        &self,
        anim_component: &EntityAnimationComponent,
        component: &mut Position,
        animation_store: &AnimationStore,
    ) {
        let (_, anim) = animation_store.float_32_animations.get(&self.uuid).unwrap();
        let range = anim.get_range(anim_component.current_frame);
        let value = range.get_value(
            anim_component.current_time - anim_component.start_time,
            anim_component.fps,
        );

        component.x = value;
    }
}
#[allow(non_camel_case_types)]
struct AnimationPropertyComponent_Position_y {
    uuid: Uuid,
}
impl AnimationPropertyComponent_Position_y {
    fn update(
        &self,
        anim_component: &EntityAnimationComponent,
        component: &mut Position,
        animation_store: &AnimationStore,
    ) {
        let (_, anim) = animation_store.float_32_animations.get(&self.uuid).unwrap();
        let range = anim.get_range(anim_component.current_frame);
        let value = range.get_value(
            anim_component.current_time - anim_component.start_time,
            anim_component.fps,
        );

        component.y = value;
    }
}

// system
#[system(for_each)]
fn entity_animation(
    commands: &mut CommandBuffer,
    entity: &Entity,
    component: &mut EntityAnimationComponent,
    #[resource] delta_time: &Duration,
) {
    let status = component.update(*delta_time);
    match status {
        EntityAnimationUpdateStatus::Playing | EntityAnimationUpdateStatus::JustFinish => (),
        EntityAnimationUpdateStatus::Finished => {
            commands.remove_component::<EntityAnimationComponent>(*entity);
            for ty in component.animation_types.iter() {
                match ty {
                    AnimationPropertyType::Position_x => {
                        commands.remove_component::<AnimationPropertyComponent_Position_x>(*entity)
                    }
                    AnimationPropertyType::Position_y => {
                        commands.remove_component::<AnimationPropertyComponent_Position_y>(*entity)
                    }
                }
            }
        }
    }
}
#[system(for_each)]
fn animation_property_component_position_x(
    anim_component: &EntityAnimationComponent,
    component: &AnimationPropertyComponent_Position_x,
    position: &mut Position,
    #[resource] animation_store: &AnimationStore,
) {
    component.update(anim_component, position, animation_store);
}
#[system(for_each)]
fn animation_property_component_position_y(
    anim_component: &EntityAnimationComponent,
    component: &AnimationPropertyComponent_Position_y,
    position: &mut Position,
    #[resource] animation_store: &AnimationStore,
) {
    component.update(anim_component, position, animation_store);
}

pub fn add_animation_system(schedule: &mut Builder) {
    schedule.add_system(entity_animation_system());
    schedule.flush();
    schedule.add_system(animation_property_component_position_x_system());
    schedule.add_system(animation_property_component_position_y_system());
}
