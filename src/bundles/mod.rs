use crate::{component::*, renderer::color};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    renderable: Renderable,
    position: Position,
    player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            renderable: Renderable {
                glyph: '@',
                fg: color::WHITE,
                bg: color::BLACK,
                z_index: 10,
            },
            position: Position { x: 10, y: 10 },
            player: Player {},
        }
    }
}

pub fn goblin(commands: &mut Commands, x: i32, y: i32) {
    commands.spawn((
        Position { x, y },
        Renderable {
            glyph: 'g',
            fg: color::RED,
            bg: color::BLACK,
            z_index: 1,
        },
    ));
}

pub fn orc(commands: &mut Commands, x: i32, y: i32) {
    commands.spawn((
        Position { x, y },
        Renderable {
            glyph: 'o',
            fg: color::RED,
            bg: color::BLACK,
            z_index: 20,
        },
    ));
}
