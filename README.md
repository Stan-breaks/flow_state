# 🌊 Flow State

> A terminal habit tracker that celebrates progress over perfection. Designed for ADHD brains that need encouragement, not judgment.

## 🧠 Why Flow State?

Traditional habit trackers break ADHD brains by:
- Resetting "streaks" after one missed day
- Overwhelming with too many options
- Focusing on perfection instead of progress
- Requiring daily consistency (impossible with executive dysfunction)

Flow State works **with** your brain, not against it.

## ✨ Core Features

- **Pattern-Based Progress**: No streaks to break, just patterns to build
- **Dual Habit Types**: Build good habits, avoid bad ones
- **Minimal Interface**: See only what matters today
- **Vim Keybindings**: Efficient, keyboard-driven navigation
- **Smart Limits**: Maximum 7 habits total (prevents overwhelm)

## 🔄 Progress System

Instead of fragile streaks, Flow State tracks your **weekly patterns**:

| Status | Days This Week | Message |
|--------|----------------|---------|
| 🔥 **Strong** | 5-7 days | You're crushing it! |
| 🎯 **Consistent** | 3-4 days | Great momentum! |
| 🌱 **Growing** | 1-2 days | Building the habit! |
| 🔄 **Fresh Start** | 0 days | Ready to begin! |

## 🎮 Interface Design

```
┌─────────────────────────────────────────┐
│              🌊 Flow State 🌊           │
│           Tuesday, June 10, 2025        │
├─────────────────────────────────────────┤
│ [Today]       [Manage]        [Stats]   │
├─────────────────────────────────────────┤
│ 🌟 Build These Habits                   │
│ ✅ [1] Take morning medication          │
│      🔥 Strong • 6/7 this week          │
│ ⚪ [2] Drink 8 glasses of water         │
│      🌱 Growing • 2/7 this week         │
│ ✅ [3] 15 min exercise/walk             │
│      🎯 Consistent • 4/7 this week      │
│                                         │
│ 🚫 Avoid These Habits                   │
│ ✅ [4] No social media before noon      │
│      🎯 Consistent • 4/7 this week      │
│ ⚪ [5] No phone during meals            │
│      🔄 Fresh Start • 0/7 this week     │
├─────────────────────────────────────────┤
│ Today's Progress: ████████░░░░ 60%      │
│ Toggle: 1-5 • Navigate: hjkl • Quit: q  │
└─────────────────────────────────────────┘
```

## ⌨️ Controls

| Key | Action |
|-----|--------|
| `1-7` | Toggle specific habit |
| `hjkl` | Navigate (Vim style) |
| `TAB` | Switch between views |
| `a` | Add habit |
| `d` | Delete habit |
| `q` | Quit |

## 🗂️ Views

1. **Today**: Daily check-in and progress
2. **Manage**: Add/edit/delete habits
3. **Stats**: Weekly and monthly patterns

## 🛠️ Technical Stack

- **Language**: Rust
- **TUI**: ratatui
- **Storage**: Local TOML files
- **Platform**: Cross-platform terminal app

## 📈 Development Phases

### Phase 1: Foundation ✅
- [x] Project vision and design
- [x] ADHD-focused UX research
- [x] Progress system design

### Phase 2: Core Implementation 🔄
- [ ] Basic TUI with navigation
- [ ] Habit management (add/edit/delete)
- [ ] Daily check-in system
- [ ] Pattern-based progress tracking
- [ ] TOML file persistence

### Phase 3: Polish 📋
- [ ] Visual themes and colors
- [ ] Statistics view
- [ ] Configuration options
- [ ] Cross-platform testing

## 🎯 Design Principles

1. **Neurodivergent-First**: Every choice considers ADHD needs
2. **Progress Over Perfection**: Any forward movement counts
3. **Minimal Cognitive Load**: Show only what's essential
4. **Keyboard Efficiency**: No mouse required
5. **Encouraging Feedback**: Celebrate every win

## 🤝 Contributing

Flow State is built by and for the neurodivergent community. Contributions welcome:

- 💡 Feature ideas and ADHD-friendly improvements
- 🐛 Testing across different ADHD experiences
- 📝 Documentation and accessibility improvements

## 📝 License

MIT License (encouraging open source adoption)

---

_"Progress, not perfection. Patterns, not streaks. Kindness, not judgment."_
