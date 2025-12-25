use crate::engine::types::{Size, Rect};

pub struct Config {
    pub window_title: &'static str,
    pub window_size: Size,
    pub game_area: Rect,
    pub tile_size: i32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            window_title: "Tanks",
            window_size: Size { w: 800, h: 600 },
            game_area: Rect { x: 0, y: 0, w: 640, h: 480 },
            tile_size: 32,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
