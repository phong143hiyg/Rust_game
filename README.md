# Tanks - Rust Edition

A complete port of the classic Tanks game from C++/SDL2 to Rust using the Macroquad game engine.

## 🎮 Features

- **Player-controlled tank** with smooth movement and firing
- **Multiple enemy types** with AI:
  - Basic Enemy (slow, weak)
  - Fast Enemy (quick movement)
  - Power Enemy (stronger bullets)
  - Armor Enemy (multiple hits required)
- **Destructible environment** with normal and steel bricks
- **Power-up system** with bonuses:
  - Extra Life
  - Shield
  - Speed Boost
  - Power Boost
- **Score tracking and lives system**
- **Progressive difficulty** (more enemies per level)
- **Eagle defense** (protect your base!)

## 🏗️ Architecture

### Idiomatic Rust Design

This port demonstrates modern Rust game development patterns:

- **No unsafe code** - All safe Rust
- **Option/Result** for error handling instead of null pointers
- **Composition over inheritance** - Structs with behavior, not class hierarchies
- **Enum-based state machine** - AppState enum for game states
- **Module system** - Replacing C++ header/source file pattern

### Project Structure

```
src/
├── main.rs              # Entry point with game loop
├── config.rs            # Game configuration
├── engine/              # Core engine abstraction
│   ├── types.rs         # Point, Rect, Direction primitives
│   ├── renderer.rs      # Drawing utilities
│   └── input.rs         # Input handling
├── objects/             # Game entities
│   ├── tank.rs          # Base tank logic
│   ├── player.rs        # Player tank
│   ├── enemy.rs         # AI-controlled tanks
│   ├── bullet.rs        # Projectiles
│   ├── brick.rs         # Destructible walls
│   ├── eagle.rs         # Base to defend
│   └── bonus.rs         # Power-ups
└── app_state/           # Game state machine
    ├── menu.rs          # Main menu
    └── game.rs          # Game logic & collision
```

## 🚀 Building & Running

### Requirements

- Rust 1.70+ (2021 edition)
- Cargo

### Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### Run

```bash
# Development
cargo run

# Release
cargo run --release
```

The executable will be in:
- `target/debug/tanks` (debug build)
- `target/release/tanks` (release build)

## 🎮 Controls

- **WASD / Arrow Keys** - Move tank
- **Space** - Fire
- **ESC** - Pause game / Return to menu
- **Enter** - Select menu option / Confirm

## 📦 Dependencies

- **macroquad 0.4** - Cross-platform game framework
  - Rendering
  - Input handling
  - Audio (for future sound effects)
  - Random number generation

## 🔄 Migration from C++

### Key Changes

| C++ (SDL2) | Rust (Macroquad) |
|------------|------------------|
| Manual memory management (`new`/`delete`) | Automatic RAII |
| Virtual functions & inheritance | Enums & pattern matching |
| Null pointers | `Option<T>` |
| Error codes | `Result<T, E>` |
| SDL_Texture | `Texture2D` |
| SDL_Event loop | `async` game loop with `.await` |
| Header/source files | Module system |

### Feature Parity

✅ All original gameplay mechanics preserved  
✅ Tank movement and collision detection  
✅ Bullet physics  
✅ Enemy AI behavior  
✅ Destructible environment  
✅ Power-up system  
✅ Score tracking  
✅ Lives system  
✅ Game states (menu, game, game over)

## 🎯 Future Enhancements

- [ ] Load levels from resource files (`resources/levels/`)
- [ ] Add texture rendering (currently using colored rectangles)
- [ ] Sound effects and music
- [ ] High score persistence
- [ ] Additional enemy AI patterns
- [ ] Multiplayer support
- [ ] Level editor

## 📝 License

This is a educational port of the original Tanks game. Refer to the original project's LICENSE for terms.

## 🙏 Credits

**Original C++ Implementation**: Tanks-master (SDL2)  
**Rust Port**: Converted to idiomatic Rust with Macroquad engine  
**Game Engine**: [Macroquad](https://github.com/not-fl3/macroquad)

---

Built with ❤️ using Rust and Macroquad
