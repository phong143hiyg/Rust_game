use crate::engine::types::{Point, Rect};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrickType {
    Normal,
    Steel,
}

pub struct Brick {
    pub position: Point,
    pub brick_type: BrickType,
    pub health: i32,
    pub is_destroyed: bool,
    pub is_indestructible: bool,
}

impl Brick {
    pub fn new(x: f32, y: f32, brick_type: BrickType) -> Self {
        Self::new_with_flags(x, y, brick_type, false)
    }
    
    pub fn new_with_flags(x: f32, y: f32, brick_type: BrickType, indestructible: bool) -> Self {
        let health = match brick_type {
            BrickType::Normal => 1,
            BrickType::Steel => 999,
        };
        
        Self {
            position: Point::new(x, y),
            brick_type,
            health,
            is_destroyed: false,
            is_indestructible: indestructible,
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        if self.is_indestructible {
            return; // Cannot damage indestructible walls
        }
        self.health -= damage;
        if self.health <= 0 {
            self.is_destroyed = true;
        }
    }
    
    pub fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            32,
            32,
        )
    }
    
    pub fn draw(&self, brick_texture: &Texture2D, stone_texture: &Texture2D) {
        let texture = match self.brick_type {
            BrickType::Normal => brick_texture,
            BrickType::Steel => stone_texture,
        };
        
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
    }
}
