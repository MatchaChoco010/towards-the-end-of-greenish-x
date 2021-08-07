use animation_engine::*;
use executor::*;

async fn game(mut cx: AnimationEngineContext) {
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
        font_name: "/font/07LogoTypeGothic-Condense.ttf".to_string(),
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

    let mut anim_name = "anim0";
    let mut bgm_name = "bgm0";
    cx.play_bgm("bgm0");

    spawn({
        let mut cx = cx.clone();
        async move {
            loop {
                cx.wait_key_down(KeyCode::X).await;
                cx.play_sfx("sfx0");
                next_frame().await;
            }
        }
    });

    spawn({
        let mut cx = cx.clone();
        async move {
            loop {
                cx.wait_key_down(KeyCode::C).await;
                if bgm_name == "bgm0" {
                    cx.play_bgm("bgm1");
                    bgm_name = "bgm1";
                } else if bgm_name == "bgm1" {
                    cx.play_bgm("bgm0");
                    bgm_name = "bgm0";
                }
                next_frame().await;
            }
        }
    });

    spawn({
        let mut cx = cx.clone();
        async move {
            cx.wait_key_down(KeyCode::Q).await;
            cx.quit();
        }
    });

    loop {
        cx.wait_key_down(KeyCode::Z).await;
        if anim_name == "anim0" {
            println!("Start anim0");
            cx.play_animation(anim_entity, "anim0").await.unwrap();
            anim_name = "anim1";
        } else if anim_name == "anim1" {
            println!("Start anim1");
            cx.play_animation(anim_entity, "anim1").await.unwrap();
            anim_name = "anim0";
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut engine = AnimationEngine::new("rust async executor and rpg!")?;
    engine.load_animation_yaml("anim0", "animation/anim0.yml")?;
    engine.load_animation_yaml("anim1", "animation/anim1.yml")?;
    engine.load_image("img0", "/image/img0.png")?;
    engine.load_bgm("bgm0", "/audio/bgm0.ogg")?;
    engine.load_bgm("bgm1", "/audio/bgm1.ogg")?;
    engine.load_sfx("sfx0", "/audio/fx0.ogg")?;
    engine.load_font(
        "/font/07LogoTypeGothic-Condense.ttf",
        "/font/07LogoTypeGothic-Condense.ttf",
    )?;
    engine.run_with_async_func(game)
}
