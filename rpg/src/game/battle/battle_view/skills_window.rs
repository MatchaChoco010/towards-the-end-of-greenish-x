use animation_engine::executor::*;
use animation_engine::*;
use futures::future::try_join_all;
use futures::{select, FutureExt};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use crate::game::battle::battle_view::*;
use crate::game_data::*;
use crate::input;

#[derive(Clone)]
pub(in super::super) struct SkillWindowItem {
    pub name_key: String,
    pub costs: Number,
    pub active: bool,
}

pub(super) struct SkillsWindow<'a> {
    cx: &'a AnimationEngineContext,
    cover: Entity,
    part_9: Entity,
    part_10: Entity,
    part_11: Entity,
    description: Entity,
    skills: Vec<SkillWindowItem>,
    skill_name_entities: Vec<Entity>,
    skill_costs: Vec<NumberView<'a>>,
}
impl<'a> SkillsWindow<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 500,
            ..Default::default()
        });
        let part_9 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-9.png".into(),
            x: 60.0,
            y: 128.0,
            z: 505,
            a: 0.0,
            ..Default::default()
        });
        let part_10 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-10.png".into(),
            z: 507,
            a: 0.0,
            ..Default::default()
        });
        let part_11 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-11.png".into(),
            x: 512.0,
            y: 68.5,
            z: 505,
            a: 0.0,
            ..Default::default()
        });
        let description = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 575.0,
            y: 150.0,
            z: 510,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let skills = vec![];
        let mut skill_name_entities = vec![];
        let mut skill_costs = vec![];
        for i in 0..11 {
            let skill_name = cx.add_text(AddTextInfo {
                font_size: 24.0,
                x: 120.0 - 44.0 * 0.0872665 * i as f32,
                y: 194.0 + 44.0 * i as f32,
                z: 510,
                a: 0.0,
                rotation: -0.0872665,
                ..Default::default()
            });
            let num = NumberView::new(
                cx,
                510.0 - 44.0 * 0.0872665 * i as f32,
                175.0 + 44.0 * i as f32,
                510,
            );
            num.set_opacity(0.0);
            skill_name_entities.push(skill_name);
            skill_costs.push(num);
        }
        Self {
            cx,
            cover,
            part_9,
            part_10,
            part_11,
            description,
            skills,
            skill_name_entities,
            skill_costs,
        }
    }

    fn set_cursor(&mut self, description: impl ToString, top_index: usize, cursor_index: usize) {
        let cursor_pos = cursor_index - top_index;
        let x = 107.0 - 0.0872665 * 44.0 * cursor_pos as f32;
        let y = 152.0 + 44.0 * cursor_pos as f32;
        self.cx.set_position(self.part_10, x, y, 507).unwrap();
        for &entity in &self.skill_name_entities {
            self.cx.set_text_key(entity, "").unwrap();
        }
        for num in &self.skill_costs {
            num.set_opacity(0.0);
        }
        for i in 0..(self.skills.len().min(11)) {
            self.cx
                .set_text_key(
                    self.skill_name_entities[i],
                    self.skills[top_index + i].name_key.to_owned(),
                )
                .unwrap();
            self.skill_costs[i].set_number(self.skills[top_index + i].costs);
            if self.skills[top_index + i].active {
                self.cx
                    .set_opacity(self.skill_name_entities[i], 1.0)
                    .unwrap();
                self.skill_costs[i].set_opacity(1.0);
            } else {
                if top_index + i == cursor_index {
                    self.cx
                        .set_opacity(self.skill_name_entities[i], 0.6)
                        .unwrap();
                    self.skill_costs[i].set_opacity(0.6);
                } else {
                    self.cx
                        .set_opacity(self.skill_name_entities[i], 0.1)
                        .unwrap();
                    self.skill_costs[i].set_opacity(0.1);
                }
            }
        }
        self.cx
            .set_text_key(self.description, description.to_string())
            .unwrap();
    }

    fn set_skills(&mut self, skills: Vec<SkillWindowItem>) {
        self.skills = skills;
    }

    async fn show(&self) {
        let mut anims: Vec<Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>> = vec![
            Box::pin(
                self.cx
                    .play_animation(self.cover, "/animation/battle/window-cover-fade-in.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_9, "/animation/battle/window-item-fade-in.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_10, "/animation/battle/window-item-fade-in.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_11, "/animation/battle/window-item-fade-in.yml"),
            ),
            Box::pin(self.cx.play_animation(
                self.description,
                "/animation/battle/window-item-fade-in.yml",
            )),
        ];
        for &entity in &self.skill_name_entities {
            anims.push(Box::pin(self.cx.play_animation(
                entity,
                "/animation/battle/window-item-nonactive-fade-in.yml",
            )));
        }
        for i in 0..self.skill_costs.len() {
            let num = &self.skill_costs[i];
            let show = self.skills.get(i).is_some();
            if show {
                anims.push(Box::pin(async move {
                    num.start_animation("/animation/battle/window-item-nonactive-fade-in.yml")
                        .await;
                    Ok(())
                }));
            } else {
                num.set_opacity(0.0);
            }
        }
        try_join_all(anims).await.expect("animation not found");
    }

    async fn hide(&self) {
        let mut anims: Vec<Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>> = vec![
            Box::pin(
                self.cx
                    .play_animation(self.cover, "/animation/battle/window-cover-fade-out.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_9, "/animation/battle/window-item-fade-out.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_10, "/animation/battle/window-item-fade-out.yml"),
            ),
            Box::pin(
                self.cx
                    .play_animation(self.part_11, "/animation/battle/window-item-fade-out.yml"),
            ),
            Box::pin(self.cx.play_animation(
                self.description,
                "/animation/battle/window-item-fade-out.yml",
            )),
        ];
        for &entity in &self.skill_name_entities {
            anims.push(Box::pin(self.cx.play_animation(
                entity,
                "/animation/battle/window-item-nonactive-fade-out.yml",
            )));
        }
        for i in 0..self.skill_costs.len() {
            let num = &self.skill_costs[i];
            let show = self.skills.get(i).is_some();
            if show {
                anims.push(Box::pin(async move {
                    num.start_animation("/animation/battle/window-item-nonactive-fade-out.yml")
                        .await;
                    Ok(())
                }));
            } else {
                num.set_opacity(0.0);
            }
        }
        try_join_all(anims).await.expect("animation not found");
    }

    pub(super) async fn select_skill<'b>(
        &mut self,
        skills: &[SkillWindowItem],
        skill_data: &[&'b SkillData],
    ) -> Option<&'b SkillData> {
        self.set_skills(skills.to_vec());

        let len = skills.len();
        let mut view_top_index = 0;
        let mut cursor_index = 0;
        self.set_cursor(
            &skill_data[cursor_index].skill_description,
            view_top_index,
            cursor_index,
        );

        self.show().await;
        self.set_cursor(
            &skill_data[cursor_index].skill_description,
            view_top_index,
            cursor_index,
        );
        let canceled = loop {
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len > 0 {
                        cursor_index = (cursor_index - 1 + len) % len;
                    }
                    if cursor_index < view_top_index {
                        view_top_index = cursor_index
                    }
                    if cursor_index > view_top_index + 10 {
                        view_top_index = cursor_index - 10
                    }
                },
                _ = input::wait_down(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len > 0 {
                        cursor_index = (cursor_index + 1 + len) % len;
                    }
                    if cursor_index < view_top_index {
                        view_top_index = cursor_index
                    }
                    if cursor_index > view_top_index + 10 {
                        view_top_index = cursor_index - 10
                    }
                },
                _ = input::wait_select_button(self.cx).fuse() => {
                    if skills[cursor_index].active {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        break false;
                    } else {
                        self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    }
                },
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    break true;
                },
            }
            self.set_cursor(
                &skill_data[cursor_index].skill_description,
                view_top_index,
                cursor_index,
            );
            delay(Duration::from_millis(150)).await;
        };
        self.hide().await;

        if canceled {
            None
        } else {
            Some(skill_data[cursor_index])
        }
    }
}
impl<'a> Drop for SkillsWindow<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.part_9);
        self.cx.delete_entity(self.part_10);
        self.cx.delete_entity(self.part_11);
        self.cx.delete_entity(self.description);
        for entity in self.skill_name_entities.drain(0..) {
            self.cx.delete_entity(entity);
        }
    }
}
