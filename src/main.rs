use macroquad::prelude::*;

mod config;
mod engine;
mod objects;
mod app_state;

use app_state::AppState;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tanks - Rust Edition".to_owned(),
        window_width: 800,
        window_height: 700,
        window_resizable: false,
        ..Default::default()
    }
}

/// Central resource manager for all game textures
pub struct Resources {
    pub player_tank: Texture2D,
    pub enemy_a: Texture2D,
    pub enemy_b: Texture2D,
    pub enemy_c: Texture2D,
    pub enemy_d: Texture2D,
    pub brick: Texture2D,
    pub stone: Texture2D,
    pub water: Texture2D,
    pub bush: Texture2D,
    pub ice: Texture2D,
    pub bonus_grenade: Texture2D,
    pub bonus_helmet: Texture2D,
    pub bonus_clock: Texture2D,
    pub bonus_shovel: Texture2D,
    pub bonus_tank: Texture2D,
    pub bonus_star: Texture2D,
    pub bonus_gun: Texture2D,
    pub bonus_boat: Texture2D,
    pub eagle: Texture2D,
    pub bullet: Texture2D,
}

impl Resources {
    async fn load() -> Self {
        let base_path = "../Tanks-master/resources/img/";
        
        Self {
            player_tank: load_texture(&format!("{}enemy_a.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, BLUE)),
            enemy_a: load_texture(&format!("{}enemy_a.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, RED)),
            enemy_b: load_texture(&format!("{}enemy_b.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, ORANGE)),
            enemy_c: load_texture(&format!("{}enemy_c.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, PURPLE)),
            enemy_d: load_texture(&format!("{}enemy_d.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, DARKGRAY)),
            brick: load_texture(&format!("{}brick.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, BROWN)),
            stone: load_texture(&format!("{}stone.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, GRAY)),
            water: load_texture(&format!("{}water.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, BLUE)),
            bush: load_texture(&format!("{}bush.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, GREEN)),
            ice: load_texture(&format!("{}ice.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, SKYBLUE)),
            bonus_grenade: load_texture(&format!("{}bonus_grenade.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, RED)),
            bonus_helmet: load_texture(&format!("{}bonus_helmet.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, BLUE)),
            bonus_clock: load_texture(&format!("{}bonus_clock.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, YELLOW)),
            bonus_shovel: load_texture(&format!("{}bonus_shovel.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, ORANGE)),
            bonus_tank: load_texture(&format!("{}bonus_tank.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, GREEN)),
            bonus_star: load_texture(&format!("{}bonus_star.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, GOLD)),
            bonus_gun: load_texture(&format!("{}bonus_gun.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, DARKGRAY)),
            bonus_boat: load_texture(&format!("{}bonus_boat.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(24, 24, SKYBLUE)),
            eagle: load_texture(&format!("{}stage_1.png", base_path))
                .await
                .unwrap_or_else(|_| Self::create_fallback_texture(32, 32, GOLD)),
            bullet: Self::create_fallback_texture(8, 8, YELLOW),
        }
    }
    
    fn create_fallback_texture(width: u16, height: u16, color: Color) -> Texture2D {
        let mut image = Image::gen_image_color(width, height, color);
        Texture2D::from_image(&image)
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load all resources
    let resources = Resources::load().await;
    
    // Initialize game state machine
    let mut current_state = AppState::Menu(app_state::MenuState::new());
    
    // Main game loop
    loop {
        // Handle input and update state
        current_state = match current_state {
            AppState::Menu(mut menu) => {
                menu.handle_input();
                menu.update(get_frame_time());
                menu.draw();
                menu.next_state()
            }
            AppState::Game(mut game) => {
                game.handle_input();
                game.update(get_frame_time());
                game.draw(&resources);
                game.next_state()
            }
            AppState::Quit => break,
        };
        
        next_frame().await;
    }
}
