use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, try_join, FutureExt};
use std::time::Duration;

use crate::game_data::*;
use crate::input;

pub(crate) struct ConfirmGetSkillWindow<'a> {
    cx: &'a AnimationEngineContext,
    part_21: Entity,
    part_22: Entity,
    part_27: Entity,
    cover: Entity,
    message: Entity,
    yes_text: Entity,
    no_text: Entity,
}
impl<'a> ConfirmGetSkillWindow<'a> {
    pub(crate) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_21 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-21.png".into(),
            x: 275.0,
            y: 485.0,
            z: 215,
            a: 0.0,
            ..Default::default()
        });
        let part_22 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-22.png".into(),
            x: 275.0,
            y: 485.0,
            z: 225,
            a: 0.0,
            ..Default::default()
        });
        let part_27 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-27.png".into(),
            x: 215.0,
            y: 350.0,
            z: 210,
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
            font_size: 30.0,
            x: 265.0,
            y: 400.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let yes_text = cx.add_text(AddTextInfo {
            font_size: 30.0,
            x: 335.0,
            y: 505.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        let no_text = cx.add_text(AddTextInfo {
            font_size: 30.0,
            x: 735.0,
            y: 505.0,
            z: 220,
            a: 0.0,
            ..Default::default()
        });
        Self {
            cx,
            part_21,
            part_22,
            part_27,
            cover,
            message,
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
                self.part_21,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_22,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_27,
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
                self.part_21,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_22,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_27,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
        )
        .expect("animation not found");
    }

    pub(crate) async fn open_get_no_skill(&self) {
        self.cx
            .set_text_key(self.message, "explore-get-no-skill-confirm-message")
            .unwrap();
        self.cx
            .set_text_key(self.yes_text, "explore-get-no-skill-confirm-yes-text")
            .unwrap();
        self.cx
            .set_text_key(self.no_text, "explore-get-no-skill-confirm-no-text")
            .unwrap();
        self.cx
            .set_position(self.part_21, 675.0, 485.0, 215)
            .unwrap();
        self.cx
            .set_position(self.part_22, 675.0, 485.0, 225)
            .unwrap();
        self.open_window_animation().await;
    }

    pub(crate) async fn open_get_skill(&self, skill_id: SkillId, skills: &Vec<SkillData>) {
        let get_skill_confirm_message = &skills
            .iter()
            .find(|s| s.id == skill_id)
            .unwrap()
            .get_skill_confirm_message;
        self.cx
            .set_text_key(self.message, get_skill_confirm_message)
            .unwrap();
        self.cx
            .set_text_key(self.yes_text, "explore-get-skill-confirm-yes-text")
            .unwrap();
        self.cx
            .set_text_key(self.no_text, "explore-get-skill-confirm-no-text")
            .unwrap();
        self.cx
            .set_position(self.part_21, 675.0, 485.0, 215)
            .unwrap();
        self.cx
            .set_position(self.part_22, 675.0, 485.0, 225)
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
                    .set_position(self.part_21, 275.0, 485.0, 215)
                    .unwrap();
                self.cx
                    .set_position(self.part_22, 275.0, 485.0, 225)
                    .unwrap();
            } else {
                self.cx
                    .set_position(self.part_21, 675.0, 485.0, 215)
                    .unwrap();
                self.cx
                    .set_position(self.part_22, 675.0, 485.0, 225)
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
impl<'a> Drop for ConfirmGetSkillWindow<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_21);
        self.cx.delete_entity(self.part_22);
        self.cx.delete_entity(self.part_27);
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.message);
        self.cx.delete_entity(self.yes_text);
        self.cx.delete_entity(self.no_text);
    }
}
