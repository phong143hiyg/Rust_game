use macroquad::prelude::*;
use crate::app_state::AppState;
use crate::engine::input::Input;
use crate::engine::types::{Point, Rect};
use crate::objects::*;
use crate::Resources;

pub struct GameState {
    player: Player,
    player2: Option<Player>,
    num_players: u8,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    bricks: Vec<Brick>,
    eagle: Eagle,
    bonuses: Vec<Bonus>,
    level: i32,
    max_level: i32,
    paused: bool,
    game_over: bool,
    victory: bool,
    collision_map: Vec<Vec<bool>>,
    total_enemies: i32,
    enemies_killed: i32,
    enemy_spawn_timer: f32,
    max_enemies_alive: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self::new_with_players(1)
    }
    
    pub fn new_with_players(num_players: u8) -> Self {
        let mut state = Self {
            player: Player::new(250.0, 400.0),
            player2: if num_players == 2 {
                Some(Player::new_with_type(350.0, 400.0, TankType::Player2))
            } else {
                None
            },
            num_players,
            enemies: Vec::new(),
            bullets: Vec::new(),
            bricks: Vec::new(),
            eagle: Eagle::new(400.0, 440.0),
            bonuses: Vec::new(),
            level: 1,
            max_level: 5,
            paused: false,
            game_over: false,
            victory: false,
            collision_map: vec![vec![false; 20]; 20],
            total_enemies: 5, // Level 1 starts with 5 enemies
            enemies_killed: 0,
            enemy_spawn_timer: 0.0,
            max_enemies_alive: 1,
        };
        
        state.load_level();
        state
    }
    
    fn load_level(&mut self) {
        // Clear existing entities
        self.enemies.clear();
        self.bullets.clear();
        self.bricks.clear();
        self.bonuses.clear();
        
        // Reset player and eagle positions
        self.player.reset_position(200.0, 400.0);
        if let Some(ref mut p2) = self.player2 {
            p2.reset_position(400.0, 400.0);
        }
        self.eagle = Eagle::new(300.0, 540.0); // Eagle at bottom
        
        // Define protected zones (no bricks allowed)
        let protected_zones = vec![
            // Player 1 spawn area
            (4, 8, 11, 14),
            // Player 2 spawn area  
            (11, 15, 11, 14),
            // Eagle area with protective walls and entrance
            (7, 12, 13, 18),
            // Enemy spawn areas
            (2, 5, 1, 3),
            (7, 10, 1, 3),
            (12, 15, 1, 3),
        ];
        
        // Create border walls (indestructible)
        for x in 0..20 {
            self.bricks.push(Brick::new_with_flags(x as f32 * 32.0, 0.0, brick::BrickType::Steel, true));
            self.bricks.push(Brick::new_with_flags(x as f32 * 32.0, 19.0 * 32.0, brick::BrickType::Steel, true));
        }
        for y in 0..20 {
            self.bricks.push(Brick::new_with_flags(0.0, y as f32 * 32.0, brick::BrickType::Steel, true));
            self.bricks.push(Brick::new_with_flags(19.0 * 32.0, y as f32 * 32.0, brick::BrickType::Steel, true));
        }
        
        // Add protective brick walls around the eagle (breakable)
        // Eagle is at tile position (9, 16) - at (300, 540) in pixels = (300/32, 540/32) = (9.375, 16.875)
        let eagle_tile_x = 9;
        let eagle_tile_y = 16;
        
        // Create a complete box of breakable bricks around the eagle
        for dx in -2..=2_i32 {
            for dy in -2..=2_i32 {
                let tx = eagle_tile_x + dx;
                let ty = eagle_tile_y + dy;
                
                // Only place bricks on the perimeter, not inside or on the eagle
                let is_perimeter = dx.abs() == 2 || dy.abs() == 2;
                let not_on_eagle = !(dx == 0 && dy == 0);
                
                if is_perimeter && not_on_eagle && tx > 0 && tx < 19 && ty > 0 && ty < 19 {
                    self.bricks.push(Brick::new(tx as f32 * 32.0, ty as f32 * 32.0, brick::BrickType::Normal));
                }
            }
        }
        
        // Add some random bricks (avoiding protected zones)
        // More bricks at higher levels
        use macroquad::rand::rand;
        let brick_count = 20 + (self.level * 5); // Increases with level
        let mut placed = 0;
        let mut attempts = 0;
        while placed < brick_count && attempts < 200 {
            attempts += 1;
            let tile_x = ((rand() % 16) + 2) as usize;
            let tile_y = ((rand() % 16) + 2) as usize;
            
            // Check if position is in a protected zone
            let mut in_protected_zone = false;
            for zone in &protected_zones {
                if tile_x >= zone.0 && tile_x <= zone.1 && tile_y >= zone.2 && tile_y <= zone.3 {
                    in_protected_zone = true;
                    break;
                }
            }
            
            if !in_protected_zone {
                let x = tile_x as f32 * 32.0;
                let y = tile_y as f32 * 32.0;
                // More steel bricks at higher levels
                let steel_chance = 20 + (self.level * 5);
                let brick_type = if rand() % 100 < steel_chance as u32 { brick::BrickType::Steel } else { brick::BrickType::Normal };
                self.bricks.push(Brick::new(x, y, brick_type));
                placed += 1;
            }
        }
        
        // Spawn enemies in safe positions
        let enemy_count = 3.min(3 + self.level);
        let enemy_spawn_positions = vec![
            (100.0, 64.0),
            (300.0, 64.0),
            (450.0, 64.0),
            (200.0, 64.0),
            (380.0, 64.0),
        ];
        
        for i in 0..enemy_count {
            let tank_type = match rand() % 4 {
                0 => TankType::BasicEnemy,
                1 => TankType::FastEnemy,
                2 => TankType::PowerEnemy,
                _ => TankType::ArmorEnemy,
            };
            
            let pos = enemy_spawn_positions[i as usize % enemy_spawn_positions.len()];
            self.enemies.push(Enemy::new(pos.0, pos.1, tank_type));
        }
        
        // Update collision map
        self.update_collision_map();
    }
    
    fn update_collision_map(&mut self) {
        self.collision_map = vec![vec![false; 20]; 20];
        
        for brick in &self.bricks {
            if !brick.is_destroyed {
                let tile_x = (brick.position.x / 32.0) as usize;
                let tile_y = (brick.position.y / 32.0) as usize;
                if tile_x < 20 && tile_y < 20 {
                    self.collision_map[tile_y][tile_x] = true;
                }
            }
        }
    }
    
    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.paused = !self.paused;
        }
        
        if self.paused || self.game_over {
            return;
        }
        
        // Player 1 movement
        if let Some(direction) = Input::get_player1_movement_direction() {
            // Calculate potential new position
            let (dx, dy) = direction.to_velocity(self.player.tank.speed * get_frame_time());
            let new_pos = Point::new(self.player.tank.position.x + dx, self.player.tank.position.y + dy);
            
            // Check if move is valid (not colliding with map or other tanks)
            let mut can_move = self.can_tank_move_to(new_pos, &self.collision_map, &self.enemies);
            
            // Also check collision with player 2
            if let Some(ref player2) = self.player2 {
                let new_bounds = Rect::new(new_pos.x as i32, new_pos.y as i32, 32, 32);
                if new_bounds.intersects(&player2.tank.get_bounds()) {
                    can_move = false;
                }
            }
            
            if can_move {
                self.player.tank.direction = direction;
                self.player.tank.position = new_pos;
            }
        }
        
        // Player 1 fire
        if Input::is_player1_fire_pressed() {
            // Check if player already has an active bullet
            let has_active_bullet = self.bullets.iter().any(|b| b.is_active && b.owner_type == TankType::Player);
            if !has_active_bullet {
                if let Some(bullet) = self.player.tank.fire() {
                    self.bullets.push(bullet);
                }
            }
        }
        
        // Player 2 movement and fire (if exists)
        let player2_new_pos = if let Some(ref player2) = self.player2 {
            Input::get_player2_movement_direction().map(|direction| {
                let (dx, dy) = direction.to_velocity(player2.tank.speed * get_frame_time());
                let new_pos = Point::new(player2.tank.position.x + dx, player2.tank.position.y + dy);
                (direction, new_pos)
            })
        } else {
            None
        };
        
        if let Some((direction, new_pos)) = player2_new_pos {
            let mut can_move = self.can_tank_move_to(new_pos, &self.collision_map, &self.enemies);
            
            // Also check collision with player 1
            let new_bounds = Rect::new(new_pos.x as i32, new_pos.y as i32, 32, 32);
            if new_bounds.intersects(&self.player.tank.get_bounds()) {
                can_move = false;
            }
            
            if can_move {
                if let Some(ref mut player2) = self.player2 {
                    player2.tank.direction = direction;
                    player2.tank.position = new_pos;
                }
            }
        }
        
        if let Some(ref mut player2) = self.player2 {
            if Input::is_player2_fire_pressed() {
                let has_active_bullet = self.bullets.iter().any(|b| b.is_active && b.owner_type == TankType::Player2);
                if !has_active_bullet {
                    if let Some(bullet) = player2.tank.fire() {
                        self.bullets.push(bullet);
                    }
                }
            }
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.paused || self.game_over {
            return;
        }
        
        // Update enemy spawn timer
        if self.enemies_killed < self.total_enemies && self.enemies.len() < self.max_enemies_alive {
            self.enemy_spawn_timer -= dt;
            if self.enemy_spawn_timer <= 0.0 {
                self.spawn_enemy();
                self.enemy_spawn_timer = 2.0; // 2 seconds between spawns
            }
        }
        
        // Update player
        self.player.update(dt);
        
        // Update player 2 if exists
        if let Some(ref mut player2) = self.player2 {
            player2.update(dt);
        };
        
        // Update enemies
        for i in 0..self.enemies.len() {
            self.enemies[i].update(dt, self.player.tank.position);
            
            // Check if enemy movement would collide with player or other enemies
            let (dx, dy) = self.enemies[i].tank.direction.to_velocity(self.enemies[i].tank.speed * dt);
            let new_pos = Point::new(self.enemies[i].tank.position.x + dx, self.enemies[i].tank.position.y + dy);
            
            // Check collision with player
            let player_bounds = self.player.tank.get_bounds();
            let new_bounds = Rect::new(new_pos.x as i32, new_pos.y as i32, 32, 32);
            let would_collide_player = new_bounds.intersects(&player_bounds);
            
            // Check collision with other enemies
            let mut would_collide_enemy = false;
            for j in 0..self.enemies.len() {
                if i != j {
                    let other_bounds = self.enemies[j].tank.get_bounds();
                    if new_bounds.intersects(&other_bounds) {
                        would_collide_enemy = true;
                        break;
                    }
                }
            }
            
            // Only move if not colliding
            if !would_collide_player && !would_collide_enemy && self.can_position_move(new_pos, &self.collision_map) {
                self.enemies[i].tank.position = new_pos;
            }
            // Only move if not colliding
            if !would_collide_player && !would_collide_enemy && self.can_position_move(new_pos, &self.collision_map) {
                self.enemies[i].tank.position = new_pos;
            }
            
            // Enemy fire - only if they don't have an active bullet
            let enemy_has_bullet = self.bullets.iter().any(|b| {
                b.is_active && matches!(b.owner_type, 
                    TankType::BasicEnemy | TankType::FastEnemy | 
                    TankType::PowerEnemy | TankType::ArmorEnemy)
            });
            
            if !enemy_has_bullet {
                if let Some(bullet) = self.enemies[i].tank.fire() {
                    self.bullets.push(bullet);
                }
            }
        }
        
        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        
        // Update bonuses
        for bonus in &mut self.bonuses {
            bonus.update(dt);
        }
        
        // Collision detection
        self.check_collisions();
        
        // Remove inactive entities
        self.bullets.retain(|b| b.is_active);
        self.enemies.retain(|e| e.tank.is_alive);
        self.bonuses.retain(|b| !b.is_collected);
        self.bricks.retain(|b| !b.is_destroyed);
        
        // Check win/lose conditions
        if self.enemies_killed >= self.total_enemies && self.enemies.is_empty() {
            // All enemies defeated - advance to next level
            if self.level < self.max_level {
                self.advance_to_next_level();
            } else {
                // Completed all levels!
                self.victory = true;
                self.game_over = true;
            }
        }
        
        if !self.eagle.is_alive || self.player.is_game_over() {
            // In 2 player mode, check if both players are out
            if let Some(ref player2) = self.player2 {
                if player2.is_game_over() {
                    self.game_over = true;
                }
            } else {
                self.game_over = true;
            }
        }
    }
    
    fn check_collisions(&mut self) {
        // Bullet vs Brick
        for bullet in &mut self.bullets {
            for brick in &mut self.bricks {
                if bullet.is_active && !brick.is_destroyed 
                    && bullet.get_bounds().intersects(&brick.get_bounds()) {
                    brick.take_damage(bullet.damage);
                    bullet.deactivate();
                }
            }
        }
        
        // Bullet vs Tank
        for bullet in &mut self.bullets {
            if !bullet.is_active {
                continue;
            }
            
            // Bullet vs Player
            if matches!(bullet.owner_type, TankType::BasicEnemy | TankType::FastEnemy 
                | TankType::PowerEnemy | TankType::ArmorEnemy) {
                if bullet.get_bounds().intersects(&self.player.tank.get_bounds()) {
                    self.player.tank.take_damage(bullet.damage);
                    bullet.deactivate();
                    
                    if !self.player.tank.is_alive {
                        self.player.lose_life();
                        if !self.player.is_game_over() {
                            self.player.reset_position(200.0, 400.0);
                        }
                    }
                }
                
                // Bullet vs Player 2
                if let Some(ref mut player2) = self.player2 {
                    if bullet.get_bounds().intersects(&player2.tank.get_bounds()) {
                        player2.tank.take_damage(bullet.damage);
                        bullet.deactivate();
                        
                        if !player2.tank.is_alive {
                            player2.lose_life();
                            if !player2.is_game_over() {
                                player2.reset_position(400.0, 400.0);
                            }
                        }
                    }
                }
            }
            
            // Bullet vs Enemies
            if matches!(bullet.owner_type, TankType::Player | TankType::Player2) {
                for enemy in &mut self.enemies {
                    if enemy.tank.is_alive && bullet.get_bounds().intersects(&enemy.tank.get_bounds()) {
                        enemy.tank.take_damage(bullet.damage);
                        bullet.deactivate();
                        
                        if !enemy.tank.is_alive {
                            self.enemies_killed += 1;
                            
                            // Only add score to the player who shot
                            if bullet.owner_type == TankType::Player {
                                self.player.add_score(100);
                            } else if bullet.owner_type == TankType::Player2 {
                                if let Some(ref mut player2) = self.player2 {
                                    player2.add_score(100);
                                }
                            }
                            
                            // Chance to spawn bonus
                            if macroquad::rand::rand() % 100 < 20 {
                                let bonus_type = match macroquad::rand::rand() % 4 {
                                    0 => BonusType::ExtraLife,
                                    1 => BonusType::Shield,
                                    2 => BonusType::Speed,
                                    _ => BonusType::Power,
                                };
                                self.bonuses.push(Bonus::new(
                                    enemy.tank.position.x,
                                    enemy.tank.position.y,
                                    bonus_type,
                                ));
                            }
                        }
                    }
                }
            }
        }
        
        // Bullet vs Eagle (only enemy bullets can destroy the eagle)
        for bullet in &mut self.bullets {
            if bullet.is_active && self.eagle.is_alive 
                && bullet.get_bounds().intersects(&self.eagle.get_bounds()) {
                // Only enemy bullets destroy the eagle
                if matches!(bullet.owner_type, TankType::BasicEnemy | TankType::FastEnemy 
                    | TankType::PowerEnemy | TankType::ArmorEnemy) {
                    self.eagle.destroy();
                }
                bullet.deactivate();
            }
        }
        
        // Player vs Bonus
        for bonus in &mut self.bonuses {
            if !bonus.is_collected && self.player.tank.get_bounds().intersects(&bonus.get_bounds()) {
                bonus.is_collected = true;
                
                match bonus.bonus_type {
                    BonusType::ExtraLife => self.player.lives += 1,
                    BonusType::Shield => self.player.tank.armor_level += 1,
                    BonusType::Speed => self.player.tank.speed = 150.0,
                    BonusType::Power => {
                        // Power up bullets
                    }
                }
            }
            
            // Player 2 vs Bonus
            if let Some(ref mut player2) = self.player2 {
                if !bonus.is_collected && player2.tank.get_bounds().intersects(&bonus.get_bounds()) {
                    bonus.is_collected = true;
                    
                    match bonus.bonus_type {
                        BonusType::ExtraLife => player2.lives += 1,
                        BonusType::Shield => player2.tank.armor_level += 1,
                        BonusType::Speed => player2.tank.speed = 150.0,
                        BonusType::Power => {
                            // Power up bullets
                        }
                    }
                }
            }
        }
        
        self.update_collision_map();
    }
    
    fn spawn_enemy(&mut self) {
        use macroquad::rand::rand;
        
        let enemy_spawn_positions = vec![
            (100.0, 64.0),
            (300.0, 64.0),
            (450.0, 64.0),
        ];
        
        let tank_type = match rand() % 4 {
            0 => TankType::BasicEnemy,
            1 => TankType::FastEnemy,
            2 => TankType::PowerEnemy,
            _ => TankType::ArmorEnemy,
        };
        
        let pos_index = (rand() as usize) % enemy_spawn_positions.len();
        let pos = enemy_spawn_positions[pos_index];
        self.enemies.push(Enemy::new(pos.0, pos.1, tank_type));
    }
    
    fn advance_to_next_level(&mut self) {
        // Advance level
        self.level += 1;
        
        // Calculate new enemy count (5 per level)
        self.total_enemies = self.level * 5;
        self.enemies_killed = 0;
        self.enemy_spawn_timer = 0.0;
        
        // Clear all entities except player
        self.enemies.clear();
        self.bullets.clear();
        self.bricks.clear();
        self.bonuses.clear();
        
        // Reset eagle
        self.eagle = Eagle::new(300.0, 400.0);
        
        // Reload the level with new random layout
        self.load_level();
    }
    
    fn can_tank_move_to(&self, pos: Point, collision_map: &[Vec<bool>], enemies: &[Enemy]) -> bool {
        let new_bounds = Rect::new(pos.x as i32, pos.y as i32, 32, 32);
        
        // Check map boundaries
        if new_bounds.x < 32 || new_bounds.y < 32 || new_bounds.x + new_bounds.w > 608 || new_bounds.y + new_bounds.h > 608 {
            return false;
        }
        
        // Check collision map (includes bricks, not destroyed ones are filtered in update_collision_map)
        if !self.can_position_move(pos, collision_map) {
            return false;
        }
        
        // Check collision with enemies (player should not overlap enemies)
        for enemy in enemies {
            if enemy.tank.is_alive && new_bounds.intersects(&enemy.tank.get_bounds()) {
                return false;
            }
        }
        
        true
    }
    
    fn can_position_move(&self, pos: Point, collision_map: &[Vec<bool>]) -> bool {
        if collision_map.is_empty() {
            return true;
        }
        
        // Check all 4 corners and center of the tank to ensure it doesn't overlap any blocks
        // This prevents tanks from moving onto or clipping through blocks
        let check_points = [
            (pos.x + 2.0, pos.y + 2.0),         // Top-left corner
            (pos.x + 30.0, pos.y + 2.0),        // Top-right corner
            (pos.x + 2.0, pos.y + 30.0),        // Bottom-left corner
            (pos.x + 30.0, pos.y + 30.0),       // Bottom-right corner
            (pos.x + 16.0, pos.y + 16.0),       // Center
        ];
        
        for (x, y) in check_points {
            let tile_x = (x / 32.0) as usize;
            let tile_y = (y / 32.0) as usize;
            
            if tile_y >= collision_map.len() || tile_x >= collision_map[0].len() {
                return false;
            }
            
            if collision_map[tile_y][tile_x] {
                return false;
            }
        }
        
        true
    }
    
    pub fn draw(&self, resources: &Resources) {
        clear_background(BLACK);
        
        // Draw game area background
        draw_rectangle(0.0, 0.0, 640.0, 640.0, Color::from_rgba(20, 20, 20, 255));
        
        // Draw bricks
        for brick in &self.bricks {
            brick.draw(&resources.brick, &resources.stone);
        }
        
        // Draw eagle
        self.eagle.draw(&resources.eagle);
        
        // Draw bonuses
        for bonus in &self.bonuses {
            bonus.draw(&resources.bonus_tank, &resources.bonus_helmet, &resources.bonus_clock, &resources.bonus_star);
        }
        
        // Draw player
        self.player.draw(&resources.player_tank);
        
        // Draw player 2 if exists
        if let Some(ref player2) = self.player2 {
            player2.draw(&resources.player_tank);
        };
        
        // Draw enemies
        for enemy in &self.enemies {
            let texture = match enemy.tank.tank_type {
                TankType::BasicEnemy => &resources.enemy_a,
                TankType::FastEnemy => &resources.enemy_b,
                TankType::PowerEnemy => &resources.enemy_c,
                TankType::ArmorEnemy => &resources.enemy_d,
                _ => &resources.enemy_a,
            };
            enemy.draw(texture);
        }
        
        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw();
        }
        
        // Draw HUD
        draw_text(&format!("P1 Lives: {}", self.player.lives), 650.0, 30.0, 20.0, WHITE);
        draw_text(&format!("P1 Score: {}", self.player.score), 650.0, 60.0, 20.0, WHITE);
        
        // Draw player 2 HUD if exists
        if let Some(ref player2) = self.player2 {
            draw_text(&format!("P2 Lives: {}", player2.lives), 650.0, 90.0, 20.0, YELLOW);
            draw_text(&format!("P2 Score: {}", player2.score), 650.0, 120.0, 20.0, YELLOW);
        }
        
        let hud_offset = if self.player2.is_some() { 150.0 } else { 90.0 };
        draw_text(&format!("Level: {}/{}", self.level, self.max_level), 650.0, hud_offset, 20.0, YELLOW);
        draw_text(&format!("Enemies: {}/{}", self.enemies_killed, self.total_enemies), 650.0, hud_offset + 30.0, 20.0, WHITE);
        draw_text(&format!("Active: {}", self.enemies.len()), 650.0, hud_offset + 60.0, 20.0, WHITE);
        
        // Draw pause overlay
        if self.paused {
            draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::from_rgba(0, 0, 0, 180));
            
            let text = "PAUSED";
            let dims = measure_text(text, None, 60, 1.0);
            draw_text(text, 320.0 - dims.width / 2.0, 280.0, 60.0, YELLOW);
            
            let text2 = "Press ESC to resume";
            let dims2 = measure_text(text2, None, 20, 1.0);
            draw_text(text2, 320.0 - dims2.width / 2.0, 340.0, 20.0, WHITE);
            
            let text3 = "Press Backspace to quit to menu";
            let dims3 = measure_text(text3, None, 20, 1.0);
            draw_text(text3, 320.0 - dims3.width / 2.0, 370.0, 20.0, WHITE);
        }
        
        // Draw game over overlay
        if self.game_over {
            draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::from_rgba(0, 0, 0, 200));
            let text = if self.victory { "VICTORY!" } else { "GAME OVER" };
            let color = if self.victory { GREEN } else { RED };
            let dims = measure_text(text, None, 60, 1.0);
            draw_text(text, 400.0 - dims.width / 2.0, 250.0, 60.0, color);
            draw_text(
                &format!("Final Score: {}", self.player.score),
                300.0,
                320.0,
                30.0,
                WHITE,
            );
            draw_text("Press ENTER to return to menu", 220.0, 370.0, 20.0, WHITE);
        }
    }
    
    pub fn next_state(self) -> AppState {
        // Check if player wants to quit to menu (Backspace when paused)
        if self.paused && is_key_pressed(KeyCode::Backspace) {
            return AppState::Menu(super::MenuState::new());
        }
        
        if self.game_over && (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape)) {
            AppState::Menu(super::MenuState::new())
        } else {
            AppState::Game(self)
        }
    }
}
