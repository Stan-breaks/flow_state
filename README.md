# Flow State

A terminal-based habit tracker designed for neurodivergent users. Focuses on progress patterns rather than perfect streaks.

<img width="1366" height="768" alt="250705_15h53m01s_screenshot" src="https://github.com/user-attachments/assets/c9f2d273-c880-4c57-8c91-32b3ba982224" />

## Features

- **Pattern-based tracking**: Weekly progress patterns instead of breakable streaks
- **Neurodivergent-friendly**: Built with ADHD considerations in mind
- **Dual habit types**: Track habits to build and habits to avoid
- **Minimal interface**: Keyboard-driven with essential information only
- **Local storage**: All data stored locally using TOML files

## Design Philosophy

Traditional habit trackers often fail neurodivergent users by:
- Resetting progress after missing a single day
- Overwhelming with excessive options and tracking
- Focusing on perfection rather than sustainable progress
- Requiring daily consistency that doesn't account for executive dysfunction

Flow State addresses these issues by emphasizing patterns over perfection and providing encouragement rather than judgment.

## Progress System

Instead of streaks, Flow State tracks weekly patterns.

- **Mastered** (5): Strong, consistent pattern
- **Established** (4): Good momentum established  
- **Developing** (3): Pattern forming
- **Struggling** (2): Frequent relapse
- **Chaotic** (1): Inconsistent

## Installation

```bash
cargo install flow_state
```

Or build from source:

```bash
git clone https://github.com/Stan-breaks/flow_state
cd flow-state
cargo build --release
```

## Usage

```bash
flow_state
```

### Basic Commands

- `flow-state` - Start the application
- `q` - Quit
- `TAB` - Switch between views (Today/Manage/Stats)
- `ENTER` - Toggle habits in Today view
- `hjkl` - Navigate (Vim-motions)

### Views

**Today View**: Daily habit check-in and progress overview
**Stats View**: Pattern health check and habit maturity

## Technical Details

- **Language**: Rust
- **TUI Library**: ratatui
- **Storage Format**: TOML
- **Platform Support**: Linux, macOS, Windows
- **Dependencies**: Minimal, focused on core functionality

## Development

### Requirements

- Rust 1.70+
- Cargo

### Building

```bash
cargo build
cargo run
```

### Contributing

Contributions welcome, especially from neurodivergent developers who understand the target use case. Please focus on:

- Accessibility improvements
- ADHD-friendly UX enhancements
- Performance optimizations
- Cross-platform compatibility


## Acknowledgments

Built with input from the neurodivergent community to create tools that work with different brain types rather than against them.
