#[cfg(feature = "logging")]
pub fn init_logger() {
    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .level_for("rpg", log::LevelFilter::Trace)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("game.log").unwrap());

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .level_for("rpg", log::LevelFilter::Info)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}

#[cfg(not(feature = "logging"))]
pub fn init_logger() {
    // do nothing
}
