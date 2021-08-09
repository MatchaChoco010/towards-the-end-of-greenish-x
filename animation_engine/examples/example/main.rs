use animation_engine::*;
use executor::*;
use futures::{select, FutureExt};

async fn game(cx: AnimationEngineContext) {
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
        key: "#hello_world".into(),
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
        let cx = cx.clone();
        async move {
            loop {
                cx.wait_key_down(KeyCode::X).await;
                cx.play_sfx("sfx0");
                next_frame().await;
            }
        }
    });

    spawn({
        let cx = cx.clone();
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
        let cx = cx.clone();
        async move {
            cx.wait_key_down(KeyCode::Q).await;
            cx.quit();
        }
    });

    spawn({
        let cx = cx.clone();
        async move {
            let mut bgm_volume: f32 = 1.0;
            loop {
                select! {
                    _ = cx.wait_key_down(KeyCode::Left).fuse() =>
                        bgm_volume = (bgm_volume - 0.1).max(0.0),
                    _ = cx.wait_key_down(KeyCode::Right).fuse() =>
                        bgm_volume = (bgm_volume + 0.1).min(1.2),
                }
                cx.set_bgm_volume(bgm_volume);
                next_frame().await;
            }
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

struct Localize;
impl animation_engine::Localize for Localize {
    fn get(&self, _key: &str) -> LocalizeText {
        LocalizeText::new(
            "07LogoTypeGothic-Condense".into(),
            "こんにちは、世界".into(),
        )
    }
}

fn main() -> anyhow::Result<()> {
    let mut engine = AnimationEngine::new("rust async executor and rpg!")?;
    engine.load_animation_yaml("anim0", "/animation/anim0.yml")?;
    engine.load_animation_yaml("anim1", "/animation/anim1.yml")?;
    engine.load_image("img0", "/image/img0.png")?;
    engine.load_bgm("bgm0", "/audio/bgm0.ogg", true)?;
    engine.load_bgm("bgm1", "/audio/bgm1.ogg", false)?;
    engine.load_sfx("sfx0", "/audio/fx0.ogg")?;
    engine.load_font(
        "07LogoTypeGothic-Condense",
        "/font/07LogoTypeGothic-Condense.ttf",
    )?;
    engine.set_localize(Box::new(Localize));
    engine.run_with_async_func(game)
}
