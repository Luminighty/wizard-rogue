use bevy::prelude::*;

use crate::renderer::{Color, ZIndex};

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Name(pub String);

impl Name {
    pub fn from<T: ToString>(name: T) -> Self {
        Self(name.to_string())
    }
}

impl ToString for Name {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub z_index: ZIndex,
}
