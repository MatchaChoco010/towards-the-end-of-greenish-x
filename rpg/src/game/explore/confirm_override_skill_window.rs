use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, try_join, FutureExt};
use std::time::Duration;

use crate::game_data::*;
use crate::input;

pub(crate) struct ConfirmOverrideSkillWindow<'a> {
    cx: &'a AnimationEngineContext,
    cover: Entity,
    part_16: Entity,
    part_17_0: Entity,
    part_17_1: Entity,
    part_18_0: Entity,
    part_18_1: Entity,
    part_19: Entity,
    part_20_0: Entity,
    part_20_1: Entity,
    part_21: Entity,
    part_22: Entity,
    message: Entity,
    current_skill_name: Entity,
    current_skill_description: Entity,
    new_skill_name: Entity,
    new_skill_description: Entity,
    yes_text: Entity,
    no_text: Entity,
}
impl<'a> ConfirmOverrideSkillWindow<'a> {
    pub(crate) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_16 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-16.png".into(),
            x: 145.0,
            y: 40.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_17_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-17.png".into(),
            x: 60.0,
            y: 200.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_17_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-17.png".into(),
            x: 670.0,
            y: 200.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_18_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-18.png".into(),
            x: 130.0,
            y: 135.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_18_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-18.png".into(),
            x: 740.0,
            y: 135.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_19 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-19.png".into(),
            x: 595.0,
            y: 325.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_20_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-20.png".into(),
            x: 390.0,
            y: 625.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_20_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-20.png".into(),
            x: 790.0,
            y: 625.0,
            z: 210,
            a: 0.0,
            ..Default::default()
        });
        let part_21 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-21.png".into(),
            x: 790.0,
            y: 625.0,
            z: 215,
            a: 0.0,
            ..Default::default()
        });
        let part_22 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-22.png".into(),
            x: 790.0,
            y: 625.0,
            z: 225,
            a: 0.0,
            ..Default::default()
        });
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            a: 0.0,
            z: 200,
            ..Default::default()
        });
        let message = cx.add_text(AddTextInfo {
            key: "explore-override-skill-confirm-message".into(),
            font_size: 24.0,
            x: 165.0,
            y: 65.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let current_skill_name = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 150.0,
            y: 155.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let current_skill_description = cx.add_text(AddTextInfo {
            font_size: 18.0,
            x: 140.0,
            y: 220.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let new_skill_name = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 760.0,
            y: 155.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let new_skill_description = cx.add_text(AddTextInfo {
            font_size: 18.0,
            x: 750.0,
            y: 220.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let yes_text = cx.add_text(AddTextInfo {
            key: "explore-override-skill-confirm-yes-text".into(),
            font_size: 36.0,
            x: 430.0,
            y: 635.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let no_text = cx.add_text(AddTextInfo {
            key: "explore-override-skill-confirm-no-text".into(),
            font_size: 36.0,
            x: 830.0,
            y: 635.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        Self {
            cx,
            cover,
            part_16,
            part_17_0,
            part_17_1,
            part_18_0,
            part_18_1,
            part_19,
            part_20_0,
            part_20_1,
            part_21,
            part_22,
            message,
            current_skill_name,
            current_skill_description,
            new_skill_name,
            new_skill_description,
            yes_text,
            no_text,
        }
    }

    async fn open_window_animation(&self) {
        try_join!(
            self.cx.play_animation(
                self.cover,
                "/animation/explore/skill-item-list-cover-fade-in.yml"
            ),
            self.cx.play_animation(
                self.message,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.yes_text,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.no_text,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_16,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_17_0,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_17_1,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_18_0,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_18_1,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_19,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_20_0,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_20_1,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_21,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_22,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.current_skill_name,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.current_skill_description,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.new_skill_name,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.new_skill_description,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
        )
        .expect("animation not found");
    }

    async fn close_window_animation(&self) {
        try_join!(
            self.cx.play_animation(
                self.cover,
                "/animation/explore/skill-item-list-cover-fade-out.yml"
            ),
            self.cx.play_animation(
                self.message,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.yes_text,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.no_text,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_16,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_17_0,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_17_1,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_18_0,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_18_1,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_19,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_20_0,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_20_1,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_21,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_22,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.current_skill_name,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.current_skill_description,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.new_skill_name,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.new_skill_description,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
        )
        .expect("animation not found");
    }

    pub(crate) async fn open_override_skill(
        &self,
        current_skill_id: SkillId,
        new_skill_id: SkillId,
        skills: &Vec<SkillData>,
    ) {
        let current_skill_name_key = &skills
            .iter()
            .find(|s| s.id == current_skill_id)
            .unwrap()
            .skill_name_with_level;
        let current_skill_description_key = &skills
            .iter()
            .find(|s| s.id == current_skill_id)
            .unwrap()
            .skill_description;
        let new_skill_name_key = &skills
            .iter()
            .find(|s| s.id == new_skill_id)
            .unwrap()
            .skill_name_with_level;
        let new_skill_description_key = &skills
            .iter()
            .find(|s| s.id == new_skill_id)
            .unwrap()
            .skill_description;
        self.cx
            .set_text_key(self.current_skill_name, current_skill_name_key)
            .unwrap();
        self.cx
            .set_text_key(
                self.current_skill_description,
                current_skill_description_key,
            )
            .unwrap();
        self.cx
            .set_text_key(self.new_skill_name, new_skill_name_key)
            .unwrap();
        self.cx
            .set_text_key(self.new_skill_description, new_skill_description_key)
            .unwrap();
        self.cx
            .set_position(self.part_21, 790.0, 625.0, 215)
            .unwrap();
        self.cx
            .set_position(self.part_22, 790.0, 625.0, 225)
            .unwrap();
        self.open_window_animation().await;
    }

    pub(crate) async fn confirm(&self) -> bool {
        let mut confirm = false;
        loop {
            select! {
                _ = input::wait_left(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    confirm = !confirm;
                }
                _ = input::wait_right(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    confirm = !confirm;
                }
                _ = input::wait_select_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/select.ogg");
                    break;
                }
            }
            if confirm {
                self.cx
                    .set_position(self.part_21, 390.0, 625.0, 215)
                    .unwrap();
                self.cx
                    .set_position(self.part_22, 390.0, 625.0, 225)
                    .unwrap();
            } else {
                self.cx
                    .set_position(self.part_21, 790.0, 625.0, 215)
                    .unwrap();
                self.cx
                    .set_position(self.part_22, 790.0, 625.0, 225)
                    .unwrap();
            }
            delay(Duration::from_millis(150)).await;
        }
        confirm
    }

    pub(crate) async fn close(&self) {
        self.close_window_animation().await;
    }
}
impl<'a> Drop for ConfirmOverrideSkillWindow<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.part_16);
        self.cx.delete_entity(self.part_17_0);
        self.cx.delete_entity(self.part_17_1);
        self.cx.delete_entity(self.part_18_0);
        self.cx.delete_entity(self.part_18_1);
        self.cx.delete_entity(self.part_19);
        self.cx.delete_entity(self.part_20_0);
        self.cx.delete_entity(self.part_20_1);
        self.cx.delete_entity(self.part_21);
        self.cx.delete_entity(self.part_22);
        self.cx.delete_entity(self.message);
        self.cx.delete_entity(self.current_skill_name);
        self.cx.delete_entity(self.current_skill_description);
        self.cx.delete_entity(self.new_skill_name);
        self.cx.delete_entity(self.new_skill_description);
        self.cx.delete_entity(self.yes_text);
        self.cx.delete_entity(self.no_text);
    }
}
