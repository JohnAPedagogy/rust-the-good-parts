# Rust - The Good Parts: Source Code

This workspace contains the complete Rust source listings from the book
*Rust - The Good Parts* by Iyalla John Alamina.

## Workspace Structure

```
gp/
├── Cargo.toml                  # Workspace definition
├── README.md                   # This file
├── guessing-game/              # Chapters 1-3: Guessing Game
│   ├── Cargo.toml              #   deps: rand, colored
│   └── src/
│       ├── main.rs             #   entry point: game::play()
│       └── game.rs             #   game logic + unit tests
├── tictactoe-ratatui/          # Chapter 4: TicTacToe (terminal UI)
│   ├── Cargo.toml              #   deps: ratatui, crossterm
│   └── src/
│       ├── main.rs             #   MVU event loop
│       └── game.rs             #   Model, View, Update + tests
├── touchtyping/                # Chapters 5-11: TouchTyping (Bevy)
│   ├── Cargo.toml              #   deps: bevy 0.18
│   ├── assets/series/          #   lesson YAML files (populate as needed)
│   └── src/
│       ├── main.rs             #   App builder + plugins
│       ├── state.rs            #   Phase, Segment, GameData
│       ├── loader.rs           #   filesystem lesson loader
│       ├── scoring.rs          #   WPM, accuracy, stars
│       ├── components/
│       │   ├── mod.rs
│       │   └── char_cell.rs    #   CharCell, CharStatus, CharCellBundle
│       └── plugins/
│           ├── mod.rs
│           ├── loader_plugin.rs
│           ├── menu.rs         #   Series/Lesson selection
│           ├── drilling.rs     #   Input, scoring, advance systems
│           └── results.rs      #   Results screen
├── examples/                   # Standalone code listings per chapter
│   ├── Cargo.toml              #   deps: rand, colored
│   └── src/
│       ├── ch01.rs             #   Chapter 1: Data
│       ├── ch02.rs             #   Chapter 2: Control
│       ├── ch03.rs             #   Chapter 3: Functions
│       ├── ch05.rs             #   Chapter 5: Structs
│       ├── ch06.rs             #   Chapter 6: Collections
│       ├── ch07.rs             #   Chapter 7: Error Handling
│       ├── ch08.rs             #   Chapter 8: Traits
│       ├── ch09.rs             #   Chapter 9: Closures
│       └── ch10.rs             #   Chapter 10: Advanced Rust
└── series/                     # Standalone code listings per blog episode
    ├── Cargo.toml              #   deps: colored
    └── src/
        ├── ep01.rs             #   Episode 1: Introducing Rust Data Types
        ├── ep02.rs             #   Episode 2: Control & Structures in Rust
        ├── ep03.rs             #   Episode 3: Adding Structure with Functions
        └── ep03_portfolio/     #   Episode 3's closing example: `portfolio`
            ├── main.rs         #     module split across two files, verbatim
            └── portfolio.rs
```

## Prerequisites

- **Rust toolchain** (edition 2021). Install via [rustup](https://rustup.rs).
- **Bevy dependencies** (for `touchtyping` only):
  - Windows: none extra (Bevy uses `wgpu`)
  - Linux: `sudo apt install pkg-config libx11-dev libasound2-dev libudev-dev`
  - macOS: none extra

## Building and Running

### All projects at once

```bash
cargo build --workspace
```

### Run a specific project

```bash
# Guessing Game (Ch1-3) — interactive terminal game
cargo run --package guessing-game

# TicTacToe with Ratatui (Ch4) — terminal UI game
cargo run --package tictactoe-ratatui

# Touch Typing with Bevy (Ch5-11) — graphical game
cargo run --package touch-typing
```

### Run a chapter's standalone examples

```bash
cargo run --bin ch01 --package examples
cargo run --bin ch02 --package examples
cargo run --bin ch03 --package examples
cargo run --bin ch05 --package examples
cargo run --bin ch06 --package examples
cargo run --bin ch07 --package examples
cargo run --bin ch08 --package examples
cargo run --bin ch09 --package examples
cargo run --bin ch10 --package examples
```

### Run a blog episode's standalone examples

```bash
cargo run --bin ep01 --package series
cargo run --bin ep02 --package series
cargo run --bin ep03 --package series

# Episode 3's closing example (the `portfolio` module split across two files)
cargo run --bin ep03_portfolio --package series
```

### Run tests

```bash
# All tests across the workspace
cargo test --workspace

# Tests for a specific project
cargo test --package guessing-game
cargo test --package tictactoe-ratatui
```

## Project Descriptions

### Guessing Game (Chapters 1-3)

A number-guessing game built progressively across three chapters:
- **Ch1**: One-round version with basic comparison
- **Ch2**: Multi-round with `loop` + `match` on `Ordering`
- **Ch3**: Refactored into functions + module (`game.rs`)

Controls: type a number and press Enter. Coloured hints via `colored` crate.

### TicTacToe (Chapter 4)

Terminal UI TicTacToe with Ratatui, following the MVU pattern:
- **Model**: `App` struct with board, player, cursor, state
- **View**: renders header, board grid, status panel
- **Update**: handles keyboard events

Controls: arrow keys to move cursor, Enter to place, `r` to restart, `q` to quit.

### Touch Typing (Chapters 5-11, Capstone)

A Bevy ECS touch-typing tutor game with multi-phase navigation.
Uses a 4-plugin architecture matching the book's design:
- **LoaderPlugin**: scans `assets/series/` for lesson files
- **MenuPlugin**: series and lesson selection menus
- **DrillingPlugin**: character grid, input handling, scoring
- **ResultsPlugin**: WPM, accuracy, star rating display

Place lesson YAML files in `touchtyping/assets/series/`.

### Blog Series Examples (Episodes 1-3)

Standalone listings from the companion "Rust the Good Parts" blog series
(distinct from the book chapters above), one file per episode, mirroring the
`examples/` crate's structure:
- **Episode 1**: Scalar types, `const`, immutability, shadowing, and a
  hand-computed Sharpe ratio, entirely inside `main`
- **Episode 2**: `if`/`match`/loops, arrays vs. `Vec`, structs, `Clone`,
  `Drop`, and the three-strategy comparison challenge
- **Episode 3**: Functions, borrowing with `&`, nested functions, modules,
  `Box<T>`/`Rc<T>`, and Episode 2's comparison refactored into a `portfolio`
  module (reproduced as its own two-file binary, `ep03_portfolio`)

## Code Provenance

All source code in this workspace was extracted verbatim from code blocks in
the *Rust — The Good Parts* markdown source files, and from the companion
blog series' markdown posts. See the book repository, and the blog series
itself, for the full text, exercises, and solutions.
