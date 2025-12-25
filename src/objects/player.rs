use crate::engine::types::{Point, Direction, Rect};
use crate::objects::{Tank, TankType};
use macroquad::prelude::*;

pub struct Player {
    pub tank: Tank,
    pub lives: i32,
    pub score: i32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            tank: Tank::new(x, y, TankType::Player),
            lives: 3,
            score: 0,
        }
    }
    
    pub fn reset_position(&mut self, x: f32, y: f32) {
        self.tank = Tank::new(x, y, TankType::Player);
    }
    
    pub fn add_score(&mut self, points: i32) {
        self.score += points;
    }
    
    pub fn lose_life(&mut self) {
        self.lives -= 1;
    }
    
    pub fn is_game_over(&self) -> bool {
        self.lives <= 0
    }
    
    pub fn update(&mut self, dt: f32) {
        self.tank.update(dt);
    }
    
    pub fn draw(&self, texture: &Texture2D) {
        self.tank.draw(texture);
    }
}
