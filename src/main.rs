use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Clone)]
struct Config {
    window_width: i32,
    window_height: i32,
    title: String,
    session: String,
}

impl Config {
    fn default() -> Self {
        Config {
            window_width: 800,
            window_height: 600,
            session: "Home".to_string(),
            title: format!("lumina | "),
        }
    }
}

fn read_config() -> Config {
    match std::fs::read_to_string("config.toml") {
        Ok(contents) => toml::from_str(&contents).unwrap_or_else(|e| {
            eprintln!("Failed to parse config.toml: {}", e);
            Config::default()
        }),
        Err(_) => {
            eprintln!("config.toml not found, using defaults");
            Config::default()
        }
    }
}

fn main() {
    let mut config = read_config();

    let title = format!("{}{}", config.title, config.session);

    let (mut rl, thread) = raylib::init()
        .size(
            config.window_width.try_into().unwrap(),
            config.window_height.try_into().unwrap(),
        )
        .title(&title)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);
    }
}
