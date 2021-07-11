use animation_engine::*;

fn main() -> anyhow::Result<()> {
    let mut engine = AnimationEngine::new()?;
    engine.load_animation_json("anim0", "animation/anim0.json")?;
    engine.load_animation_json("anim1", "animation/anim1.json")?;
    engine.load_image("img0", "/image/img0.png")?;

    let cx = engine.get_context();
    let _entity = cx.add_image(AddImageInfo {
        name: "img0".to_string(),
        x: 400.0 - 64.0,
        y: 300.0 - 64.0,
        z: 10,
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.8,
        scale: 0.25,
        ..Default::default()
    });
    let _ = cx.add_text(AddTextInfo {
        text: "こんにちは、世界".to_string(),
        x: 400.0 - 100.0,
        y: 300.0 + 64.0,
        z: 15,
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.75,
        ..Default::default()
    });
    let anim_entity = cx.add_rect(AddRectInfo {
        width: 50.0,
        height: 50.0,
        r: 1.0,
        g: 1.0,
        b: 0.0,
        ..Default::default()
    });

    let mut checker: Option<AnimationFinishChecker> = None;
    let mut anim_name = "anim0";
    engine.run_with_update_func(move |cx| {
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
    })
}
