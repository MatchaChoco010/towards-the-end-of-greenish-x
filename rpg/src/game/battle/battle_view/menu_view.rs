use animation_engine::*;
use futures::try_join;

pub(super) struct MenuView<'a> {
    cx: &'a AnimationEngineContext,
    part_2: Entity,
    part_3: Entity,
    part_4: Entity,
    attack: Entity,
    skills: Entity,
    items: Entity,
    player_info: Entity,
    enemy_info: Entity,
}
impl<'a> MenuView<'a> {
    pub(super) fn new(cx: &'a AnimationEngineContext) -> Self {
        let part_2 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-2.png".into(),
            y: 215.0,
            z: 365,
            a: 0.0,
            ..Default::default()
        });
        let part_3 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-3.png".into(),
            y: 191.0,
            z: 370,
            a: 0.0,
            ..Default::default()
        });
        let part_4 = cx.add_image(AddImageInfo {
            name: "/image/ui/battle-part-4.png".into(),
            y: 232.5,
            z: 375,
            a: 0.0,
            ..Default::default()
        });
        let attack = cx.add_text(AddTextInfo {
            key: "battle-menu-attack".into(),
            font_size: 36.0,
            x: 55.0,
            y: 240.0,
            z: 380,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let skills = cx.add_text(AddTextInfo {
            key: "battle-menu-skills".into(),
            font_size: 36.0,
            x: 50.287609,
            y: 294.0,
            z: 380,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let items = cx.add_text(AddTextInfo {
            key: "battle-menu-items".into(),
            font_size: 36.0,
            x: 45.575218,
            y: 348.0,
            z: 380,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let player_info = cx.add_text(AddTextInfo {
            key: "battle-menu-player-info".into(),
            font_size: 36.0,
            x: 40.862827,
            y: 402.0,
            z: 380,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        let enemy_info = cx.add_text(AddTextInfo {
            key: "battle-menu-enemy-info".into(),
            font_size: 36.0,
            x: 36.150436,
            y: 456.0,
            z: 380,
            a: 0.0,
            rotation: -0.0872665,
            ..Default::default()
        });
        Self {
            cx,
            part_2,
            part_3,
            part_4,
            attack,
            skills,
            items,
            player_info,
            enemy_info,
        }
    }

    pub(super) fn set_cursor(&self, index: usize) {
        let x = 33.0 - 0.0872665 * 54.0 * index as f32;
        let y = 211.0 + 54.0 * index as f32;
        self.cx.set_position(self.part_4, x, y, 375).unwrap();
    }

    pub(super) fn set_active(&self, active: bool) {
        if active {
            self.cx.set_opacity(self.part_4, 1.0).unwrap();
            self.cx.set_opacity(self.attack, 1.0).unwrap();
            self.cx.set_opacity(self.skills, 1.0).unwrap();
            self.cx.set_opacity(self.items, 1.0).unwrap();
            self.cx.set_opacity(self.player_info, 1.0).unwrap();
            self.cx.set_opacity(self.enemy_info, 1.0).unwrap();
        } else {
            self.cx.set_opacity(self.part_4, 0.3).unwrap();
            self.cx.set_opacity(self.attack, 0.3).unwrap();
            self.cx.set_opacity(self.skills, 0.3).unwrap();
            self.cx.set_opacity(self.items, 0.3).unwrap();
            self.cx.set_opacity(self.player_info, 0.3).unwrap();
            self.cx.set_opacity(self.enemy_info, 0.3).unwrap();
        }
    }

    pub(super) async fn show(&self) {
        try_join!(
            self.cx
                .play_animation(self.part_2, "/animation/battle/menu-window-shadow-show.yml"),
            self.cx
                .play_animation(self.part_3, "/animation/battle/menu-window-show.yml"),
            self.cx
                .play_animation(self.attack, "/animation/battle/menu-attack-show.yml"),
            self.cx
                .play_animation(self.skills, "/animation/battle/menu-skills-show.yml"),
            self.cx
                .play_animation(self.items, "/animation/battle/menu-items-show.yml"),
            self.cx.play_animation(
                self.player_info,
                "/animation/battle/menu-player-info-show.yml"
            ),
            self.cx.play_animation(
                self.enemy_info,
                "/animation/battle/menu-enemy-info-show.yml"
            ),
        )
        .expect("animation not found");
        self.cx
            .play_animation(self.part_4, "/animation/battle/menu-cursor-show.yml")
            .await
            .expect("animation not found");
    }

    pub(super) async fn hide(&self) {
        self.cx.set_opacity(self.part_4, 0.0).unwrap();
        try_join!(
            self.cx
                .play_animation(self.part_2, "/animation/battle/menu-window-shadow-hide.yml"),
            self.cx
                .play_animation(self.part_3, "/animation/battle/menu-window-hide.yml"),
            self.cx
                .play_animation(self.attack, "/animation/battle/menu-attack-hide.yml"),
            self.cx
                .play_animation(self.skills, "/animation/battle/menu-skills-hide.yml"),
            self.cx
                .play_animation(self.items, "/animation/battle/menu-items-hide.yml"),
            self.cx.play_animation(
                self.player_info,
                "/animation/battle/menu-player-info-hide.yml"
            ),
            self.cx.play_animation(
                self.enemy_info,
                "/animation/battle/menu-enemy-info-hide.yml"
            ),
        )
        .expect("animation not found");
    }
}
impl<'a> Drop for MenuView<'a> {
    fn drop(&mut self) {
        self.cx.delete_entity(self.part_2);
        self.cx.delete_entity(self.part_3);
        self.cx.delete_entity(self.part_4);
        self.cx.delete_entity(self.attack);
        self.cx.delete_entity(self.skills);
        self.cx.delete_entity(self.items);
        self.cx.delete_entity(self.player_info);
        self.cx.delete_entity(self.enemy_info);
    }
}
