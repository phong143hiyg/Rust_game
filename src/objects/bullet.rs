use crate::engine::types::{Point, Direction, Rect};
use crate::objects::TankType;
use macroquad::prelude::*;

pub struct Bullet {
    pub position: Point,
    pub direction: Direction,
    pub speed: f32,
    pub is_active: bool,
    pub owner_type: TankType,
    pub damage: i32,
}

impl Bullet {
    pub fn new(x: f32, y: f32, direction: Direction, owner_type: TankType) -> Self {
        let (speed, damage) = match owner_type {
            TankType::Player | TankType::Player2 => (200.0, 1),  // Player bullet speed (300/1.5)
            TankType::PowerEnemy => (66.0, 2),  // Enemy bullets 3x slower (200/3)
            _ => (66.0, 1),  // Other enemies 3x slower than player
        };
        
        Self {
            position: Point::new(x, y),
            direction,
            speed,
            is_active: true,
            owner_type,
            damage,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        let (dx, dy) = self.direction.to_velocity(self.speed * dt);
        self.position.x += dx;
        self.position.y += dy;
        
        // Deactivate if out of bounds (map is 640x640)
        if self.position.x < 0.0 || self.position.x > 640.0
            || self.position.y < 0.0 || self.position.y > 640.0 {
            self.is_active = false;
        }
    }
    
    pub fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            8,
            8,
        )
    }
    
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
    
    pub fn draw(&self) {
        let color = match self.owner_type {
            TankType::Player => YELLOW,
            _ => WHITE,
        };
        
        draw_circle(
            self.position.x + 4.0,
            self.position.y + 4.0,
            4.0,
            color,
        );
    }
}
