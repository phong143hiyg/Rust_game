pub mod menu;
pub mod game;

pub use menu::MenuState;
pub use game::GameState;

pub enum AppState {
    Menu(MenuState),
    Game(GameState),
    Quit,
}
