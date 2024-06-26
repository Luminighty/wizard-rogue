// Terminal dimensions in characters
pub const TERMINAL_WIDTH: usize = 30;
pub const TERMINAL_HEIGHT: usize = 30;

// Character dimensions in pixels
pub const CHAR_WIDTH: usize = 8;
pub const CHAR_HEIGHT: usize = 8;

// Character Map dimensions in characters
pub const CHAR_MAP_WIDTH: usize = 16;
pub const CHAR_MAP_HEIGHT: usize = 16;

// Window scaling
pub const SCALE: f32 = 2.0;

// Actual Window dimensions in pixels
pub const WIN_WIDTH: f32 = (TERMINAL_WIDTH * CHAR_WIDTH) as f32 * SCALE;
pub const WIN_HEIGHT: f32 = (TERMINAL_HEIGHT * CHAR_HEIGHT) as f32 * SCALE;
