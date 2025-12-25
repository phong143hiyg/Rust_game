use crate::engine::types::{Point, Direction};
use crate::objects::{Tank, TankType};
use macroquad::prelude::*;

pub struct Enemy {
    pub tank: Tank,
    pub ai_timer: f32,
    pub ai_update_interval: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, tank_type: TankType) -> Self {
        Self {
            tank: Tank::new(x, y, tank_type),
            ai_timer: 0.0,
            ai_update_interval: 1.0,
        }
    }
    
    pub fn update(&mut self, dt: f32, player_pos: Point) {
        self.tank.update(dt);
        self.ai_timer += dt;
        
        // Update AI every interval
        if self.ai_timer >= self.ai_update_interval {
            self.ai_timer = 0.0;
            self.update_ai(player_pos);
        }
        
        // Don't move here - movement handled in game state with collision detection
    }
    
    fn update_ai(&mut self, player_pos: Point) {
        use macroquad::rand::gen_range;
        use macroquad::rand::rand;
        
        // Simple AI: 70% chance to move toward player, 30% random
        if rand() % 100 < 70 {
            // Move toward player
            let dx = player_pos.x - self.tank.position.x;
            let dy = player_pos.y - self.tank.position.y;
            
            if dx.abs() > dy.abs() {
                self.tank.direction = if dx > 0.0 { Direction::Right } else { Direction::Left };
            } else {
                self.tank.direction = if dy > 0.0 { Direction::Down } else { Direction::Up };
            }
        } else {
            // Random direction
            let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
            self.tank.direction = directions[(rand() % 4) as usize];
        }
        
        // Random fire
        if rand() % 100 < 30 {
            let _ = self.tank.fire();
        }
    }
    
    pub fn draw(&self, texture: &Texture2D) {
        self.tank.draw(texture);
    }
}
