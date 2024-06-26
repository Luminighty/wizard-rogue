use bevy::prelude::*;
use bevy_pixels::{schedule::Draw, PixelsOptions, PixelsPlugin, PixelsWrapper};

use crate::{config, renderer::terminal::render_terminal};

use super::{glyphset::read_glyphset_file, terminal::Terminal};

#[derive(Default)]
pub struct TerminalBuilder {
    width: usize,
    height: usize,
    glyphs_file: String,
    char_width: usize,
    char_height: usize,
    glyph_map_width: usize,
    glyph_map_height: usize,
}

impl TerminalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // The amount of characters the terminal can have per row/column
    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    // The amount of characters found per row/column
    pub fn glyphs(mut self, file: String, map_width: usize, map_height: usize) -> Self {
        self.glyphs_file = file;
        self.glyph_map_width = map_width;
        self.glyph_map_height = map_height;
        self
    }

    pub fn char_size(mut self, width: usize, height: usize) -> Self {
        self.char_width = width;
        self.char_height = height;
        self
    }

    pub fn build(self) -> TerminalPlugin {
        TerminalPlugin {
            width: self.width,
            height: self.height,
            glyphs_file: self.glyphs_file,
            glyph_map_width: self.glyph_map_width,
            glyph_map_height: self.glyph_map_height,
            char_height: self.char_height,
            char_width: self.char_width,
        }
    }
}

pub struct TerminalPlugin {
    width: usize,
    height: usize,
    glyphs_file: String,
    char_width: usize,
    char_height: usize,
    glyph_map_width: usize,
    glyph_map_height: usize,
}

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let glyphset = read_glyphset_file(
            &self.glyphs_file,
            (self.glyph_map_width, self.glyph_map_height),
            (self.char_width, self.char_height),
        );
        let terminal = Terminal::new(glyphset, self.width, self.height);
        app.add_plugins(self.get_pixels_plugin())
            .insert_resource(terminal)
            .add_systems(Draw, main_render_terminal);
    }
}

impl TerminalPlugin {
    fn get_pixels_plugin(&self) -> PixelsPlugin {
        let primary_window = Some(PixelsOptions {
            width: (self.width * self.char_width) as u32,
            height: (self.height * self.char_height) as u32,
            auto_resize_buffer: true,
            scale_factor: config::SCALE,
            ..Default::default()
        });
        PixelsPlugin { primary_window }
    }
}

fn main_render_terminal(terminal: Res<Terminal>, mut pixels: Query<&mut PixelsWrapper>) {
    let Ok(mut pixels) = pixels.get_single_mut() else {
        return;
    };

    render_terminal(terminal.into_inner(), pixels.pixels.frame_mut());
}
