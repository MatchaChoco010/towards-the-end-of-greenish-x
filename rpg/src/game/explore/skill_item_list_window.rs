use animation_engine::executor::*;
use animation_engine::*;
use futures::{join, select, try_join, FutureExt};
use log::trace;
use std::time::Duration;

use crate::game::explore::*;
use crate::game::*;
use crate::game_data::*;
use crate::input;

pub(super) struct SkillItemListWindow<'a> {
    cx: &'a AnimationEngineContext,
    cover: Entity,
    part_0: Entity,
    part_1: Entity,
    part_2: Entity,
    part_3: Entity,
    part_4: Entity,
    message_text: Entity,
    header_text: Entity,
    description: Entity,
    list: Vec<Entity>,
    confirm_get_skill: ConfirmGetSkillWindow<'a>,
    confirm_override_skill: ConfirmOverrideSkillWindow<'a>,
}
impl<'a> SkillItemListWindow<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let cover = cx.add_rect(AddRectInfo {
            width: 1280.0,
            height: 720.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            z: 100,
            ..Default::default()
        });
        let part_0 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-12.png".into(),
            x: 38.0,
            y: 200.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let part_1 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-13.png".into(),
            z: 115,
            a: 0.0,
            ..Default::default()
        });
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-14.png".into(),
            x: 500.0,
            y: 148.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-15.png".into(),
            x: 125.0,
            y: 110.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let part_4 = cx.add_image(AddImageInfo {
            name: "/image/ui/explore-part-16.png".into(),
            x: 140.0,
            y: 20.0,
            z: 110,
            a: 0.0,
            ..Default::default()
        });
        let message_text = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 170.0,
            y: 45.0,
            z: 120,
            a: 0.0,
            ..Default::default()
        });
        let header_text = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 190.0,
            y: 140.0,
            z: 120,
            a: 0.0,
            ..Default::default()
        });
        let description = cx.add_text(AddTextInfo {
            font_size: 24.0,
            x: 610.0,
            y: 180.0,
            z: 120,
            a: 0.0,
            ..Default::default()
        });
        let list = (0..15)
            .map(|i| {
                cx.add_text(AddTextInfo {
                    font_size: 24.0,
                    x: 135.0 - 30.0 * i as f32 * 0.1763269807,
                    y: 230.0 + 30.0 * i as f32,
                    z: 120,
                    ..Default::default()
                })
            })
            .collect();
        Self {
            cx,
            cover,
            part_0,
            part_1,
            part_2,
            part_3,
            part_4,
            message_text,
            header_text,
            description,
            list,
            confirm_get_skill: ConfirmGetSkillWindow::new(cx),
            confirm_override_skill: ConfirmOverrideSkillWindow::new(cx),
        }
    }

    pub(super) async fn open_window_animation(&self) {
        try_join!(
            self.cx.play_animation(
                self.cover,
                "/animation/explore/skill-item-list-cover-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_0,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_2,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_3,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.part_4,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.message_text,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.header_text,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            self.cx.play_animation(
                self.description,
                "/animation/explore/skill-item-list-fade-in.yml"
            ),
            futures::future::join_all(self.list.iter().map(|&entity| {
                self.cx
                    .play_animation(entity, "/animation/explore/skill-item-list-fade-in.yml")
            }))
            .map(|_| Ok(())),
        )
        .expect("animation not found");
    }

    pub(super) async fn close_window_animation(&self) {
        self.cx.set_opacity(self.part_1, 0.0).unwrap();
        try_join!(
            self.cx.play_animation(
                self.cover,
                "/animation/explore/skill-item-list-cover-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_0,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_2,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_3,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.part_4,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.message_text,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.header_text,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            self.cx.play_animation(
                self.description,
                "/animation/explore/skill-item-list-fade-out.yml"
            ),
            futures::future::join_all(self.list.iter().map(|&entity| {
                self.cx
                    .play_animation(entity, "/animation/explore/skill-item-list-fade-out.yml")
            }))
            .map(|_| Ok(())),
        )
        .expect("animation not found");
    }

    fn set_cursor(&self, view_top_index: usize, cursor_index: usize, len: usize) {
        let i = cursor_index - view_top_index;
        self.cx
            .set_position(
                self.part_1,
                125.0 - 30.0 * i as f32 * 0.1763269807,
                220.0 + 30.0 * i as f32,
                115,
            )
            .unwrap();
        if len > 0 {
            self.cx.set_opacity(self.part_1, 1.0).unwrap();
        } else {
            self.cx.set_opacity(self.part_1, 0.0).unwrap();
        }
    }

    fn set_new_skills(
        &self,
        view_top_index: usize,
        cursor_index: usize,
        skill_id_list: &[usize],
        player_data: &PlayerData,
    ) {
        for index in view_top_index..(view_top_index + 15) {
            if let Some(skill) = skill_id_list.get(index) {
                let key = player_data
                    .skills
                    .iter()
                    .find(|s| &s.id == skill)
                    .unwrap()
                    .skill_name_with_level
                    .to_owned();
                self.cx.set_text_key(self.list[index], key).unwrap();
            } else {
                if index == skill_id_list.len() {
                    self.cx
                        .set_text_key(self.list[index], "explore-get-no-skill")
                        .unwrap()
                } else {
                    self.cx.set_text_key(self.list[index], "").unwrap();
                }
            }
        }
        if let Some(skill) = skill_id_list.get(cursor_index) {
            let key = player_data
                .skills
                .iter()
                .find(|s| &s.id == skill)
                .unwrap()
                .skill_description
                .to_owned();
            self.cx.set_text_key(self.description, key).unwrap();
        } else {
            if cursor_index == skill_id_list.len() {
                self.cx
                    .set_text_key(self.description, "explore-get-no-skill-description")
                    .unwrap()
            } else {
                self.cx.set_text_key(self.description, "").unwrap();
            }
        }
    }

    fn set_owned_skills(
        &self,
        view_top_index: usize,
        cursor_index: usize,
        player_state: &PlayerState,
        player_data: &PlayerData,
    ) {
        let mut skills = player_state.get_skills();
        skills.sort_by_key(|skill| {
            player_data
                .skills
                .iter()
                .find(|s| &s.id == skill)
                .unwrap()
                .skill_type
        });
        for index in view_top_index..(view_top_index + 15) {
            if let Some(skill) = skills.get(index) {
                let key = player_data
                    .skills
                    .iter()
                    .find(|s| &s.id == skill)
                    .unwrap()
                    .skill_name_with_level
                    .to_owned();
                self.cx.set_text_key(self.list[index], key).unwrap();
            } else {
                self.cx.set_text_key(self.list[index], "").unwrap();
            }
        }
        if let Some(skill) = skills.get(cursor_index) {
            let key = player_data
                .skills
                .iter()
                .find(|s| &s.id == skill)
                .unwrap()
                .skill_description
                .to_owned();
            self.cx.set_text_key(self.description, key).unwrap();
        } else {
            self.cx.set_text_key(self.description, "").unwrap();
        }
    }

    pub(super) async fn show_add_skill(
        &self,
        player_state: &mut PlayerState,
        player_data: &PlayerData,
        skill_id_list: &[usize],
    ) {
        trace!("Open add skill menu");

        self.cx.play_sfx("/audio/sfx/menu.ogg");

        self.cx
            .set_text_key(self.message_text, "explore-add-skill")
            .unwrap();
        self.cx
            .set_text_key(self.header_text, "explore-header-new-skill")
            .unwrap();

        let mut view_top_index = [0, 0];
        let mut cursor_index = [0, 0];
        let mut page = 0;
        let len = [skill_id_list.len() + 1, player_state.get_skills().len()];

        self.set_cursor(view_top_index[page], cursor_index[page], len[page]);
        self.set_new_skills(
            view_top_index[page],
            cursor_index[page],
            skill_id_list,
            player_data,
        );

        self.open_window_animation().await;

        loop {
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len[page] > 0 {
                        cursor_index[page] = (cursor_index[page] - 1 + len[page]) % len[page];
                    }
                    if cursor_index[page] < view_top_index[page] {
                        view_top_index[page] = cursor_index[page]
                    }
                    if cursor_index[page] > view_top_index[page] + 15 {
                        view_top_index[page] = cursor_index[page] - 15
                    }
                },
                _ = input::wait_down(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len[page] > 0 {
                        cursor_index[page] = (cursor_index[page] + 1 + len[page]) % len[page];
                    }
                    if cursor_index[page] < view_top_index[page] {
                        view_top_index[page] = cursor_index[page]
                    }
                    if cursor_index[page] > view_top_index[page] + 15 {
                        view_top_index[page] = cursor_index[page] - 15
                    }
                },
                _ = input::wait_left(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    page = (page + 1) % 2;
                },
                _ = input::wait_right(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    page = (page + 1) % 2;
                },
                _ = input::wait_select_button(self.cx).fuse() => {
                    if page == 0 {
                        self.cx.play_sfx("/audio/sfx/select.ogg");
                        self.cx.play_sfx("/audio/sfx/menu.ogg");
                        if cursor_index[page] == skill_id_list.len() {
                            // confirm get no skill
                            self.confirm_get_skill.open_get_no_skill().await;
                            if self.confirm_get_skill.confirm().await {
                                join!(
                                    self.confirm_get_skill.close(),
                                    self.close_window_animation(),
                                );
                                break;
                            } else {
                                self.confirm_get_skill.close().await;
                            }
                        }else {
                            if let Some(override_skill) = player_state.is_skill_override(
                                skill_id_list[cursor_index[page]],
                                &player_data.skills,
                            ) {
                                // confirm override skill
                                self.confirm_override_skill.open_override_skill(
                                    override_skill,
                                    skill_id_list[cursor_index[page]],
                                    &player_data.skills,
                                ).await;
                                if self.confirm_override_skill.confirm().await {
                                    join!(
                                        self.confirm_override_skill.close(),
                                        self.close_window_animation(),
                                    );
                                    player_state.add_skill(
                                        skill_id_list[cursor_index[page]],
                                        &player_data.skills,
                                    );
                                    break;
                                } else {
                                    self.confirm_override_skill.close().await;
                                }
                            } else {
                                // confirm get skill
                                self.confirm_get_skill.open_get_skill(
                                    skill_id_list[cursor_index[page]],
                                    &player_data.skills,
                                ).await;
                                if self.confirm_get_skill.confirm().await {
                                    join!(
                                        self.confirm_get_skill.close(),
                                        self.close_window_animation(),
                                    );
                                    player_state.add_skill(
                                        skill_id_list[cursor_index[page]],
                                        &player_data.skills,
                                    );
                                    break;
                                } else {
                                    self.confirm_get_skill.close().await;
                                }
                            }
                        }
                    }
                },
            }
            if page == 0 {
                self.cx
                    .set_text_key(self.header_text, "explore-header-new-skill")
                    .unwrap();
                self.set_new_skills(
                    view_top_index[page],
                    cursor_index[page],
                    skill_id_list,
                    player_data,
                );
            } else {
                self.cx
                    .set_text_key(self.header_text, "explore-header-owned-skills")
                    .unwrap();
                self.set_owned_skills(
                    view_top_index[page],
                    cursor_index[page],
                    player_state,
                    player_data,
                );
            }
            self.set_cursor(view_top_index[page], cursor_index[page], len[page]);
            delay(Duration::from_millis(150)).await;
        }
    }
    fn set_owned_items(
        &self,
        view_top_index: usize,
        cursor_index: usize,
        player_state: &PlayerState,
        item_data: &Vec<ItemData>,
    ) {
        let mut items = player_state.get_items();
        items.sort_by_key(|(item, _)| item_data.iter().find(|i| &i.id == item).unwrap().id);
        for index in view_top_index..(view_top_index + 15) {
            if let Some((item, count)) = items.get(index) {
                let key = item_data
                    .iter()
                    .find(|i| &i.id == item)
                    .unwrap()
                    .item_name_with_count
                    .to_owned();
                self.cx.set_text_key(self.list[index], key).unwrap();
                self.cx
                    .set_text_format_args(self.list[index], &[&count.to_string()])
                    .unwrap();
            } else {
                self.cx.set_text_key(self.list[index], "").unwrap();
            }
        }
        if let Some((item, _)) = items.get(cursor_index) {
            let key = item_data
                .iter()
                .find(|i| &i.id == item)
                .unwrap()
                .item_description
                .to_owned();
            self.cx.set_text_key(self.description, key).unwrap();
        } else {
            self.cx.set_text_key(self.description, "").unwrap();
        }
    }

    pub(super) async fn show_skills_and_items(
        &self,
        player_state: &PlayerState,
        player_data: &PlayerData,
        item_data: &Vec<ItemData>,
    ) {
        trace!("Open skills and items menu");

        self.cx.play_sfx("/audio/sfx/menu.ogg");

        self.cx
            .set_text_key(self.message_text, "explore-check-skills-and-items")
            .unwrap();
        self.cx
            .set_text_key(self.header_text, "explore-header-owned-skills")
            .unwrap();

        let mut view_top_index = [0, 0];
        let mut cursor_index = [0, 0];
        let mut page = 0;
        let len = [
            player_state.get_skills().len(),
            player_state.get_items().len(),
        ];

        self.set_cursor(view_top_index[page], cursor_index[page], len[page]);
        self.set_owned_skills(
            view_top_index[page],
            cursor_index[page],
            player_state,
            player_data,
        );

        self.open_window_animation().await;

        loop {
            select! {
                _ = input::wait_up(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len[page] > 0 {
                        cursor_index[page] = (cursor_index[page] - 1 + len[page]) % len[page];
                    }
                    if cursor_index[page] < view_top_index[page] {
                        view_top_index[page] = cursor_index[page]
                    }
                    if cursor_index[page] > view_top_index[page] + 15 {
                        view_top_index[page] = cursor_index[page] - 15
                    }
                },
                _ = input::wait_down(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    if len[page] > 0 {
                        cursor_index[page] = (cursor_index[page] + 1 + len[page]) % len[page];
                    }
                    if cursor_index[page] < view_top_index[page] {
                        view_top_index[page] = cursor_index[page]
                    }
                    if cursor_index[page] > view_top_index[page] + 15 {
                        view_top_index[page] = cursor_index[page] - 15
                    }
                },
                _ = input::wait_left(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    page = (page + 1) % 2;
                },
                _ = input::wait_right(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cursor.ogg");
                    page = (page + 1) % 2;
                },
                _ = input::wait_cancel_button(self.cx).fuse() => {
                    self.cx.play_sfx("/audio/sfx/cancel.ogg");
                    break;
                },
            }
            if page == 0 {
                self.cx
                    .set_text_key(self.header_text, "explore-header-owned-skills")
                    .unwrap();
                self.set_owned_skills(
                    view_top_index[page],
                    cursor_index[page],
                    player_state,
                    player_data,
                );
            } else {
                self.cx
                    .set_text_key(self.header_text, "explore-header-owned-items")
                    .unwrap();
                self.set_owned_items(
                    view_top_index[page],
                    cursor_index[page],
                    player_state,
                    item_data,
                );
            }
            self.set_cursor(view_top_index[page], cursor_index[page], len[page]);
            delay(Duration::from_millis(150)).await;
        }

        self.close_window_animation().await;
    }
}
impl<'a> Drop for SkillItemListWindow<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.cover);
        self.cx.delete_entity(self.part_0);
        self.cx.delete_entity(self.part_1);
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
        self.cx.delete_entity(self.part_4);
        self.cx.delete_entity(self.message_text);
        self.cx.delete_entity(self.header_text);
        self.cx.delete_entity(self.description);
        for &item in self.list.iter() {
            self.cx.delete_entity(item);
        }
    }
}
