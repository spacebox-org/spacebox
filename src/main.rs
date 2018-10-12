extern crate chrono;
extern crate fern;
extern crate ignore;
extern crate log;
extern crate notify;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

use fern::colors::{Color, ColoredLevelConfig};

/// Handles the configuration file format.
pub mod config;

/// Defines the overarching error type for Spacebox
pub mod error;
pub use error::{Error, Result};

pub mod backend;

/// The entry point to the daemon.  Runs an event handling loop.
pub fn main() {
    configure_logging().expect("Failed to configure logging");
    let config_file =
        std::fs::read_to_string(get_config_path()).expect("Failed to read config file.");
    let app_config: config::ApplicationConfig =
        serde_yaml::from_str::<config::ApplicationConfig>(&config_file)
            .expect("Failed to deserialize the config file");
    println!("Hello, world!");
}

fn get_config_path() -> std::path::PathBuf {
    "$HOME/.config/spacebox/config.yaml".into()
}

fn configure_logging() -> std::result::Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::Cyan)
        .debug(Color::Green)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        }).level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
