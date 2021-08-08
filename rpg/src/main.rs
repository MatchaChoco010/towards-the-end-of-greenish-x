use animation_engine::*;

mod assets_load;
mod game;
mod init_logger;
mod input;

fn main() -> anyhow::Result<()> {
    init_logger::init_logger();
    let mut engine = AnimationEngine::new("Towards The End of Greenish-X")?;
    assets_load::load(&mut engine)?;
    engine.run_with_async_func(game::game)
}
