use animation_engine::*;

mod assets_load;
mod game;
mod game_data;
mod init_logger;
mod input;
mod localization;
mod save_data;

fn main() -> anyhow::Result<()> {
    init_logger::init_logger();

    let mut engine = AnimationEngine::new("Towards The End of Greenish-X")?;
    assets_load::load(&mut engine)?;
    localization::set_localize(&mut engine);

    let global_data = game::GlobalData::load(&mut engine)?;
    let game = game::game(global_data);

    engine.run_with_async_func(game)
}
