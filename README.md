# Flow State

A terminal-based habit tracker designed for neurodivergent users. Focuses on progress patterns rather than perfect streaks.
| | |
|---|---|
| ![home screen image](assets/image2.png) | ![heatmap image](assets/image1.png) |

## Features

- **Pattern-based tracking**: Weekly progress patterns instead of breakable streaks
- **Neurodivergent-friendly**: Built with ADHD considerations in mind
- **Dual habit types**: Track habits to build and habits to avoid
- **Minimal interface**: Keyboard-driven with essential information only
- **Local storage**: All data stored locally using TOML files
- **Optional notifications**: Made to avoid alert fatigue while still providing a reminder

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

## Notification system

Notifications are a double-edged sword for people with ADHD - we need them to avoid forgetting important things, but 
when we're juggling multiple tasks in a busy life, they tend to overwhelm our neurodivergent brains.

That's why the notification system in flow_state is designed to avoid throwing useless data at you. In the config
file `.config/flow_state/notification.toml`, you can set up a daily reminder. This reminder will fire at a time
defined by keys `hour` and `minute`, once a day. But it will only fire in two circumstances:
- when you've forgotten to check your tracker entirely, a non-judgmental message will remind you to take a look.
  This is going to fire only when your task completion percentage is under a set threshold `low_threshold`.
- when you've done exceptionally well, a cheering message will give you some encouragement for knocking it out of
  the park. This is as a reward counterbalance to the reminder message. This will fire when your task completion is
  above `high_threshold`.

As a result, there will be no alerts on most days you actually check off things on the tracker, when your
completion isn't near zero or near perfect. But you will receive a single reminder if you've forgotten to look at your
tasks, and some well-deserved congrats for doing well.

Neurodivergent-friendly defaults for notifications are in example file `config/notification.toml`. Without this file with
key `enable` set to `true`, the notifications are off - before enabling them, make sure they are useful for your
particular neurodivergent brain.

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
- `y` - Switch between editing Today and Yesterday, in case you forgot to mark things off
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
