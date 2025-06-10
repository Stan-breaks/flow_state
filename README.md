# 🌊 Flow State

> An ADHD-friendly terminal-based habit tracker built with Rust and designed for neurodivergent minds.

## 🎯 Project Vision

Flow State is a beautiful, minimal TUI (Terminal User Interface) habit tracker specifically designed to work **with** ADHD brains, not against them. Instead of punishing missed days or overwhelming with options, it celebrates patterns and progress.

## 🧠 Why ADHD-Focused?

Traditional habit trackers often fail for ADHD users because they:

- Break "streaks" after one missed day (devastating for rejection-sensitive brains)
- Overwhelm with too many options and data points
- Require consistent daily engagement (difficult with executive dysfunction)
- Focus on perfection rather than progress

Flow State addresses these pain points with thoughtful design choices.

## 📋 Current Features (Design Phase)

### ✨ Core Functionality

- **Dual Habit Types**: Track positive habits to build AND negative habits to avoid
- **Forgiving Progress System**: Pattern-based tracking instead of fragile streaks
- **Minimal Cognitive Load**: Clean, focused interface showing only what matters today
- **Vim Keybindings**: Efficient navigation for power users
- **Smart Limits**: 5-10 good habits, 2-5 bad habits (prevents overwhelm)

### 🎨 ADHD-Friendly Design Elements

- **Visual Dopamine Hits**: Bright colors and clear checkmarks for completion
- **Encouraging Language**: "Building momentum" instead of "streak broken"
- **Flexible Progress Tracking**: Allows for real life interruptions
- **Quick Keyboard Access**: Number keys for instant habit toggling
- **Motivational Feedback**: Celebrates partial progress

## 🔄 Progress System Innovation

Instead of traditional streaks that reset to zero, Flow State uses a **pattern-based approach**:

| Status            | Emoji    | Weekly Habits          | Bi-weekly Habits          |
| ----------------- | -------- | ---------------------- | ------------------------- |
| 🔥 **Strong**     | Fire     | 6-7 out of last 7 days | 12-14 out of last 14 days |
| 🎯 **Consistent** | Target   | 4-5 out of last 7 days | 8-11 out of last 14 days  |
| 🌱 **Growing**    | Seedling | 2-3 out of last 7 days | 4-7 out of last 14 days   |
| 📈 **Building**   | Chart    | 1 out of last 7 days   | 1-3 out of last 14 days   |
| 🔄 **Restarting** | Refresh  | Ready to begin again   | Fresh start available     |
| 🌙 **Resting**    | Moon     | Recovery period        | Taking a break            |

This system:

- ✅ Uses consistent 7-day (calendar-aligned)
- ✅ Celebrates consistency over perfection
- ✅ Allows missed days without penalty
- ✅ Uses positive, growth-oriented language
- ✅ Adapts messaging when patterns struggle

## 🎮 User Interface Design

### Main Screen Layout

```
┌─────────────────────────────────────────┐
│              🌊 Flow State 🌊           │
│           Tuesday, June 10, 2025        │
├─────────────────────────────────────────┤
│ [Today]       [Manage]        [stats]   │
├─────────────────────────────────────────┤
│ 🌟 Good Habits (5/10)                   │
│ ✅ [1] Take morning medication          │
│      🔥 Strong • 6/7 this week          │
│ ⚪ [2] Drink 8 glasses of water         │
│      🌱 Growing • 3/7 this week         │
│ ✅ [3] 15 min exercise/walk             │
│      🎯 Consistent • 4/7 this week      │
│ ⚪ [4] Deep clean one area              │
│      🌱 Growing • 3/7     weekly        │
│ ⚪ [5] Call family/friends              │
│      🔄 Restarting • 1/7  weekly        │
│                                         │
│ 🚫 Habits to Avoid (2/5)                │
│ ✅ [6] No social media before noon      │
│      🎯 Consistent • 5/7 this week      │
│ ⚪ [7] No phone during meals            │
│      🌙 Resting • 0/7 this week         │
├─────────────────────────────────────────┤
│ Progress: ████████████░░░░░░░░ 43%      │
│ Navigation: hjkl • Toggle: 1-9 • q quit │
└─────────────────────────────────────────┘
```

### Vim Keybindings

| Key        | Action                      |
| ---------- | --------------------------- |
| `hjkl`     | Navigate up/down/left/right |
| `gg` / `G` | Jump to top/bottom          |
| `1-9`      | Toggle specific habit       |
| `SPACE`    | Toggle selected habit       |
| `TAB`      | Switch between views        |
| `a`        | Add new habit               |
| `d`        | Delete habit                |
| `q`        | Quit application            |

## 📈 Development Journey

### Phase 1: Ideation ✅

**Goal**: Define the problem and vision

- ✅ Identified ADHD-specific pain points with existing habit trackers
- ✅ Designed forgiving progress system
- ✅ Created UI mockup with ADHD-friendly elements
- ✅ Established habit limits (5-10 good, 2-5 bad)

### Phase 2: Technical Design 🔄

**Goal**: Plan the Rust implementation

- [ ] Choose TUI library (ratatui vs cursive vs tui-rs)
- [ ] Design data structures for habits and progress
- [ ] Plan file-based persistence strategy
- [ ] Architect vim keybinding system
- [ ] Design state management for different views

### Phase 3: Core Implementation 📋

**Goal**: Build MVP functionality

- [ ] Basic TUI with tab navigation
- [ ] Habit creation and management
- [ ] Daily check-in system
- [ ] Pattern-based progress calculation
- [ ] Data persistence (JSON/TOML files)

### Phase 4: Polish & Features 📋

**Goal**: Enhance user experience

- [ ] Color themes and visual polish
- [ ] Week/month view implementations
- [ ] Statistics and insights
- [ ] Import/export functionality
- [ ] Optional system notifications

## 🛠️ Technical Stack

- **Language**: Rust 🦀
- **TUI Framework**: ratatui
- **Data Storage**: Local files TOML
- **Configuration**: TOML config files
- **Platform**: Cross-platform (Windows, macOS, Linux)

## 🎨 Design Philosophy

1. **Neurodivergent-First**: Every design decision considers ADHD/neurodivergent needs
2. **Progress Over Perfection**: Celebrate any forward movement
3. **Minimal Cognitive Load**: Show only what's needed, when it's needed
4. **Keyboard-Driven**: Efficient interaction without mouse dependency
5. **Visually Rewarding**: Clear feedback for dopamine-seeking brains

## 🤝 Contributing

This project is in active development! While we're still in the design phase, contributions and feedback are welcome:

- 💡 **Ideas**: Share ADHD-friendly features or improvements
- 🎨 **Design**: Suggest UI/UX enhancements
- 🐛 **Testing**: Help test with different ADHD experiences
- 📝 **Documentation**: Improve guides and explanations

## 📝 License

[License TBD - likely MIT or Apache 2.0]

## 🙏 Acknowledgments

Built with love for the ADHD and neurodivergent community. Special thanks to everyone who struggles with traditional productivity tools - this is for us.

---

_"Progress, not perfection. Patterns, not streaks. Kindness, not judgment."_

