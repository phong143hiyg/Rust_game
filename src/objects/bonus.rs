use crate::engine::types::{Point, Rect};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonusType {
    ExtraLife,
    Shield,
    Speed,
    Power,
}

pub struct Bonus {
    pub position: Point,
    pub bonus_type: BonusType,
    pub is_collected: bool,
    pub lifetime: f32,
}

impl Bonus {
    pub fn new(x: f32, y: f32, bonus_type: BonusType) -> Self {
        Self {
            position: Point::new(x, y),
            bonus_type,
            is_collected: false,
            lifetime: 10.0, // 10 seconds
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            self.is_collected = true; // Remove expired bonuses
        }
    }
    
    pub fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            24,
            24,
        )
    }
    
    pub fn draw(&self, tank_texture: &Texture2D, helmet_texture: &Texture2D, clock_texture: &Texture2D, star_texture: &Texture2D) {
        let texture = match self.bonus_type {
            BonusType::ExtraLife => tank_texture,
            BonusType::Shield => helmet_texture,
            BonusType::Speed => clock_texture,
            BonusType::Power => star_texture,
        };
        
        draw_texture_ex(
            texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(24.0, 24.0)),
                ..Default::default()
            },
        );
    }
}
