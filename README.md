# Rust Pong Game

A classic Pong game implementation in Rust using the GGEZ game framework.

## Features

- Single-player gameplay against AI opponent
- Realistic ball physics and paddle collisions
- Human-like AI behavior with reaction delays and imperfect tracking
- Score tracking with win condition at 3 points
- Visual feedback for scoring and game state
- Smooth paddle movement and ball animation


## Controls

- **Up Arrow**: Move your paddle up
- **Down Arrow**: Move your paddle down
- **S**: Start game
- **R**: Reset game
- **E**: Exit game


## How to Play

1. Use Up/Down arrow keys to control your paddle (left side)
2. Block the ball from passing your paddle
3. Score by getting the ball past the AI's paddle
4. First to score 3 points wins
5. Ball is served toward the last point winner


## System Requirements

- Rust (2021 edition or later)
- GGEZ game framework dependencies
- OpenGL 3.2 support


## Installation

Install Rust:

Download from: https://www.rust-lang.org/tools/install.
Run rustup-init.exe.
Follow installer prompts.
Note: Git needs to be installed on your system. If not installed, download from https://git-scm.com/downloads

1. Clone the repository:
```bash
git clone https://github.com/Prudhveeraj123/ping_pong_game.git
cd ping_pong_game
```

2. Build and run:
```bash
cargo build --release
cargo run
```


## Technical Details

- Built with Rust and GGEZ game framework
- Uses collision detection for paddle/ball interactions
- Implements AI with randomized behavior for realistic gameplay
- Smooth game state management with countdown timers
- Custom graphics rendering with score display
