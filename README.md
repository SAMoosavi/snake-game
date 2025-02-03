# Snake Game in Rust

A simple Snake game implemented in Rust using TUI (Text-based User Interface). This project demonstrates how to use Rust for game development with terminal-based rendering.

## Features
- Classic Snake gameplay
- Terminal-based UI using `ratatui`
- Real-time keyboard input handling
- Smooth game loop using `tokio`
- Score tracking

## Play Game

### Clone the repository

```sh
git clone https://github.com/SAMoosavi/snake_game.git
cd snake_game
```

### Build and Run

```sh
cargo run --release
```

## Controls
- **Arrow Keys**: Move the snake (Up, Down, Left, Right, h, j, k, l)
- **Q**: Quit the game

## Dependencies
This project uses the following Rust crates:
- `ratatui` - Terminal-based UI rendering
- `crossterm` - Handling user input
- `tokio` - Async runtime for game loop

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## License
This project is licensed under the MIT License.

---
Enjoy the game! 🎮

