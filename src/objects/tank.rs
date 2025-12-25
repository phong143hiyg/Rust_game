use crate::engine::types::{Point, Direction, Rect};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TankType {
    Player,
    BasicEnemy,
    FastEnemy,
    PowerEnemy,
    ArmorEnemy,
}

pub struct Tank {
    pub position: Point,
    pub direction: Direction,
    pub tank_type: TankType,
    pub speed: f32,
    pub health: i32,
    pub max_health: i32,
    pub is_alive: bool,
    pub fire_cooldown: f32,
    pub armor_level: i32,
}

impl Tank {
    pub fn new(x: f32, y: f32, tank_type: TankType) -> Self {
        let (speed, health, armor) = match tank_type {
            TankType::Player => (66.0, 3, 0),  // 100 / 1.5
            TankType::BasicEnemy => (33.0, 1, 0),  // 50 / 1.5
            TankType::FastEnemy => (100.0, 1, 0),  // 150 / 1.5
            TankType::PowerEnemy => (33.0, 1, 0),
            TankType::ArmorEnemy => (33.0, 4, 3),
        };
        
        Self {
            position: Point::new(x, y),
            direction: Direction::Up,
            tank_type,
            speed,
            health,
            max_health: health,
            is_alive: true,
            fire_cooldown: 0.0,
            armor_level: armor,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.fire_cooldown > 0.0 {
            self.fire_cooldown -= dt;
        }
    }
    
    pub fn move_in_direction(&mut self, direction: Direction, dt: f32, collision_map: &[Vec<bool>]) {
        self.direction = direction;
        let (dx, dy) = direction.to_velocity(self.speed * dt);
        let new_pos = Point::new(self.position.x + dx, self.position.y + dy);
        
        if self.can_move_to(new_pos, collision_map) {
            self.position = new_pos;
        }
    }
    
    pub fn turn(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }
    
    pub fn can_fire(&self) -> bool {
        self.is_alive && self.fire_cooldown <= 0.0
    }
    
    pub fn fire(&mut self) -> Option<super::Bullet> {
        if !self.can_fire() {
            return None;
        }
        
        // Different cooldowns for player vs enemies
        self.fire_cooldown = match self.tank_type {
            TankType::Player => 0.5,  // 500ms cooldown
            _ => 2.0,  // 2 second cooldown for enemies
        };
        
        // Calculate bullet spawn position (in front of tank)
        let (offset_x, offset_y) = match self.direction {
            Direction::Up => (12.0, -8.0),
            Direction::Down => (12.0, 32.0),
            Direction::Left => (-8.0, 12.0),
            Direction::Right => (32.0, 12.0),
        };
        
        Some(super::Bullet::new(
            self.position.x + offset_x,
            self.position.y + offset_y,
            self.direction,
            self.tank_type,
        ))
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        if self.armor_level > 0 {
            self.armor_level -= 1;
        } else {
            self.health -= damage;
            if self.health <= 0 {
                self.is_alive = false;
            }
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
    
    fn can_move_to(&self, pos: Point, collision_map: &[Vec<bool>]) -> bool {
        let bounds = Rect::new(pos.x as i32, pos.y as i32, 32, 32);
        
        // Check map boundaries
        if bounds.x < 0 || bounds.y < 0 {
            return false;
        }
        
        if collision_map.is_empty() {
            return true;
        }
        
        // Check collision map
        let tile_x = (bounds.x / 32) as usize;
        let tile_y = (bounds.y / 32) as usize;
        
        if tile_y >= collision_map.len() || tile_x >= collision_map[0].len() {
            return false;
        }
        
        !collision_map[tile_y][tile_x]
    }
    
    pub fn draw(&self, texture: &Texture2D) {
        let rotation = self.direction.to_angle();
        
        // Draw texture centered on position
        draw_texture_ex(
            texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                rotation: rotation.to_radians(),
                ..Default::default()
            },
        );
        
        // Draw health bar
        if self.health < self.max_health {
            let bar_width = 32.0 * (self.health as f32 / self.max_health as f32);
            draw_rectangle(
                self.position.x,
                self.position.y - 5.0,
                bar_width,
                3.0,
                GREEN,
            );
        }
        
        // Draw armor indicator
        if self.armor_level > 0 {
            draw_circle(
                self.position.x + 28.0,
                self.position.y + 4.0,
                3.0,
                YELLOW,
            );
        }
    }
}
