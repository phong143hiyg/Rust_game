use crate::engine::types::{Point, Rect};
use macroquad::prelude::*;

pub struct Eagle {
    pub position: Point,
    pub is_alive: bool,
}

impl Eagle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Point::new(x, y),
            is_alive: true,
        }
    }
    
    pub fn destroy(&mut self) {
        self.is_alive = false;
    }
    
    pub fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            32,
            32,
        )
    }
    
    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(
            texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        
        if !self.is_alive {
            // Draw X when destroyed
            draw_line(
                self.position.x + 8.0,
                self.position.y + 8.0,
                self.position.x + 24.0,
                self.position.y + 24.0,
                3.0,
                RED,
            );
            draw_line(
                self.position.x + 24.0,
                self.position.y + 8.0,
                self.position.x + 8.0,
                self.position.y + 24.0,
                3.0,
                RED,
            );
        }
    }
}
