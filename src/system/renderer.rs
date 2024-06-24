use bevy::prelude::*;

use crate::{
    component::{Position, Renderable},
    renderer::terminal::Terminal,
};

pub fn render(objects: Query<(&Position, &Renderable)>, mut terminal: ResMut<Terminal>) {
    terminal.clear();
    for (pos, renderable) in &objects {
        terminal.put(
            pos.x,
            pos.y,
            renderable.glyph as u8,
            renderable.fg,
            renderable.bg,
        )
    }
}
