use env_logger;
use log::LevelFilter;
use std::io::Write;

pub fn init_logging() {
    let mut builder = env_logger::builder();
    let level = std::env::var("RUST_LOG")
        .map(|s| s.parse().unwrap_or(LevelFilter::Info))
        .unwrap_or(LevelFilter::Info);
    builder
        .filter_level(level)
        .format(|buf, record| writeln!(buf, "{}", record.args()));
    builder.init();
}
