use animation_engine::*;

mod assets_load;
mod game;
mod init_logger;
mod input;
mod localization;
mod save_data;

fn main() -> anyhow::Result<()> {
    init_logger::init_logger();
    let mut engine = AnimationEngine::new("Towards The End of Greenish-X")?;
    assets_load::load(&mut engine)?;
    localization::set_localize(&mut engine);
    engine.run_with_async_func(game::game)
}
