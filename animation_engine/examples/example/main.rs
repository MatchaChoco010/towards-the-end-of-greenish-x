use animation_engine::*;

fn main() -> anyhow::Result<()> {
    let mut engine = AnimationEngine::new()?;
    let cx = engine.get_context();

    cx.load_animation_json("anim0", std::path::Path::new("animation/anim0.json"))?;
    cx.load_animation_json("anim1", std::path::Path::new("animation/anim1.json"))?;

    let mut counter = 0;
    let mut entity = cx.add_rect(0.0, 0.0, 0, 50.0, 50.0);
    let mut x = 0.0;
    let mut y = 0.0;
    let anim_entity = cx.add_rect(0.0, 0.0, 1, 80.0, 80.0);
    let mut checker: Option<AnimationFinishChecker> = None;
    let mut anim_name = "anim0";
    engine.run_with_update_func(move |cx| {
        cx.delete_entity(entity);

        if cx.key_pressed(KeyCode::Right) {
            x += 10.0;
        } else if cx.key_pressed(KeyCode::Left) {
            x -= 10.0;
        }

        if cx.key_pressed(KeyCode::Down) {
            y += 10.0;
        } else if cx.key_pressed(KeyCode::Up) {
            y -= 10.0;
        }

        let width = 50.0 + (0.05 * counter as f32).sin() * 50.0;

        entity = cx.add_rect(x % 800.0, y % 600.0, 0, width, 50.0);

        if cx.key_down(KeyCode::Z) {
            match checker.as_mut() {
                None => {
                    println!("Start anim0");
                    checker = cx.start_animation(anim_entity, "anim0").ok();
                    anim_name = "anim1";
                }
                Some(c) => {
                    if c.is_finished() {
                        if anim_name == "anim0" {
                            println!("Start anim0");
                            checker = cx.start_animation(anim_entity, "anim0").ok();
                            anim_name = "anim1";
                        } else if anim_name == "anim1" {
                            println!("Start anim1");
                            checker = cx.start_animation(anim_entity, "anim1").ok();
                            anim_name = "anim0";
                        }
                    }
                }
            }
        }

        counter += 1;
    })
}
