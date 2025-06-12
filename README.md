# Flow State

A terminal-based habit tracker designed for neurodivergent users. Focuses on progress patterns rather than perfect streaks.

## Features

- **Pattern-based tracking**: Weekly progress patterns instead of breakable streaks
- **Neurodivergent-friendly**: Built with ADHD considerations in mind
- **Dual habit types**: Track habits to build and habits to avoid
- **Flexible scheduling**: Daily, weekly, and custom frequencies
- **Pause system**: Life-aware habit management for difficult periods
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

Instead of streaks, Flow State tracks weekly patterns using a rolling 7-day window:

- **Mastering** (85-100%): Strong, consistent pattern
- **Building** (60-84%): Good momentum established  
- **Growing** (25-59%): Pattern forming
- **Starting** (0-24%): Beginning phase

## Installation

```bash
cargo install flow-state
```

Or build from source:

```bash
git clone https://github.com/Stan-breaks/flow_state
cd flow-state
cargo build --release
```

## Usage

### Basic Commands

- `flow-state` - Start the application
- `q` - Quit
- `TAB` - Switch between views (Today/Manage/Stats)
- `1-7` - Toggle habits in Today view
- `hjkl` - Navigate (Vim-motions)

### Views

**Today View**: Daily habit check-in and progress overview
**Manage View**: Add, edit, delete, and pause habits  
**Stats View**: Weekly and monthly progress patterns

### Habit Management

- Maximum 7 active habits to prevent overwhelm
- Context fields for time, location, and prerequisites
- Pause system for life disruptions
- Flexible frequency options (daily/weekly/custom)

## Data Storage

All data is stored locally in `~/.config/flow-state/`:

```
~/.config/flow-state/
├── config.toml       # Application settings
├── habits.toml       # Habit definitions
├── data/             # Monthly progress data
└── backups/          # Automatic backups
```

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
cargo test
cargo run
```

### Contributing

Contributions welcome, especially from neurodivergent developers who understand the target use case. Please focus on:

- Accessibility improvements
- ADHD-friendly UX enhancements
- Performance optimizations
- Cross-platform compatibility

## License

MIT License

## Acknowledgments

Built with input from the neurodivergent community to create tools that work with different brain types rather than against them.
