# 🎯 Habit Tracker

A terminal-based habit tracker built in Rust to help you build and maintain good habits!

## Features

- ✅ **Add/Remove Habits** - Manage your habit list
- 🎯 **Daily Tracking** - Mark habits as complete each day
- 🔥 **Streak Counter** - See your current streak for each habit
- 📊 **Statistics** - View weekly and monthly completion stats
- 💾 **Persistent Storage** - Data saved in JSON format
- 🎨 **ASCII Progress Bars** - Visual progress indicators

## Installation

1. Make sure you have Rust installed (https://rustup.rs/)
2. Clone or download this project
3. Build and run:

```bash
cd habit_tracker
cargo run
```

## Usage

### Commands

- `add <habit_name>` - Add a new habit
- `remove <habit_name>` - Remove a habit
- `done <habit_name>` - Mark habit as done for today
- `list` - Show all habits with stats
- `stats <habit_name>` - Show detailed statistics
- `help` - Show available commands
- `quit` - Exit the program

### Example Session

```
🎯 Welcome to Habit Tracker!
Type 'help' for commands

> add Workout
✅ Added habit 'Workout'

> add Meditation
✅ Added habit 'Meditation'

> add Reading
✅ Added habit 'Reading'

> done Workout
✅ Marked 'Workout' as done for today! 🎉

> done Reading
✅ Marked 'Reading' as done for today! 🎉

> list

📊 YOUR HABITS
============================================================

[✓] Workout (🔥 1 day streak)
    Week:  ██░░░░░░░░ 1/7
    Month: 1 completions

[✓] Reading (🔥 1 day streak)
    Week:  ██░░░░░░░░ 1/7
    Month: 1 completions

[ ] Meditation (🔥 0 day streak)
    Week:  ░░░░░░░░░░ 0/7
    Month: 0 completions

> stats Workout

📈 STATS FOR: Workout
========================================
🔥 Current streak: 1 days
📅 Created: 2025-01-31
✅ Total completions: 1
📊 Last 7 days: 1
📊 Last 30 days: 1

Recent completions:
  • 2025-01-31
```

## Rust Concepts Demonstrated

### Structs & Enums
- `Habit` struct to represent individual habits
- `HabitTracker` struct for managing the collection

### File I/O
- JSON serialization/deserialization with `serde`
- Persistent storage in `habits.json`

### Time Handling
- Uses `chrono` for date operations
- Streak calculation
- Weekly/monthly statistics

### State Management
- HashMap for storing habits
- Load/save functionality
- Mutable state updates

### CLI Interaction
- Interactive command loop
- User input parsing
- Pretty-printed output with emojis

### Additional Features
- ASCII progress bars (█░)
- Sorting habits by streak
- Date-based filtering

## Data Storage

Habits are stored in `habits.json` in the current directory. The file is automatically created on first run and updated after each change.

Example data structure:
```json
{
  "habits": {
    "Workout": {
      "name": "Workout",
      "created_date": "2025-01-31",
      "completions": [
        "2025-01-31",
        "2025-02-01"
      ]
    }
  }
}
```

## Building for Release

```bash
cargo build --release
```

The compiled binary will be in `target/release/habit_tracker`

## Future Enhancements

Some ideas to extend the project:
- [ ] Custom goals (e.g., 4 times per week)
- [ ] Habit categories/tags
- [ ] Export data to CSV
- [ ] Reminder notifications
- [ ] Color-coded output
- [ ] Calendar view
- [ ] Longest streak tracking

## License

MIT
