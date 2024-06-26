use bevy::{prelude::*, window::WindowResolution};
use renderer::plugin::*;

pub mod bundles;
pub mod component;
pub mod config;
pub mod renderer;
pub mod system;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin()), terminal_plugin()))
        .add_systems(Startup, system::spawn_entities)
        .add_systems(Update, system::renderer::render)
        .add_systems(Update, system::player::input)
        .run();
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Wizard Rogue".to_string(),
            resolution: WindowResolution::new(config::WIN_WIDTH, config::WIN_HEIGHT),
            ..default()
        }),
        ..default()
    }
}

fn terminal_plugin() -> TerminalPlugin {
    TerminalBuilder::new()
        .size(config::TERMINAL_WIDTH, config::TERMINAL_HEIGHT)
        .char_size(config::CHAR_WIDTH, config::CHAR_HEIGHT)
        .glyphs(
            "pastiche_8x8.png".to_string(),
            config::CHAR_MAP_WIDTH,
            config::CHAR_MAP_HEIGHT,
        )
        .build()
}
