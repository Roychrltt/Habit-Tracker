# 🎯 Habit Tracker v2.0

A powerful terminal-based habit tracker built in Rust to help you build and maintain good habits!

## ✨ Features

### Core Features
- ✅ **Add/Remove Habits** - Manage your habit list
- 🎯 **Custom Goals** - Daily, weekly (e.g., 4x/week), or monthly targets
- 📁 **Categories/Tags** - Organize habits by category (Fitness, Learning, etc.)
- 🔥 **Streak Tracking** - Current streak + longest streak ever
- 📊 **Statistics** - Comprehensive weekly and monthly stats
- 💾 **Persistent Storage** - Data saved in JSON format

### Visual Features
- 🎨 **Color-Coded Output** - Green for success, yellow for warnings, red for alerts
- 📊 **ASCII Progress Bars** - Beautiful visual progress indicators
- 📅 **Calendar View** - Monthly calendar showing completion days in green
- 🎯 **Goal Status** - Visual indicators for goal completion

### Advanced Features
- 📤 **CSV Export** - Export all your data for analysis
- ⏰ **Reminder Notifications** - Set time-based reminders (Linux/Mac)
- 🏆 **Longest Streak** - Track your personal best for each habit
- 📈 **Completion Rates** - Percentage-based progress tracking

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

### Basic Commands

- `add <habit_name> [goal]` - Add a new habit with optional goal
  - Examples: `add Workout`, `add Reading 4 weekly`, `add Meditation 20 monthly`
- `remove <habit_name>` - Remove a habit
- `done <habit_name>` - Mark habit as done for today
- `list [category]` - Show all habits (optionally filter by category)
- `stats <habit_name>` - Show detailed statistics

### Advanced Commands

- `category <habit_name> <category>` - Set category for a habit
- `categories` - List all categories
- `calendar <habit_name>` - Show monthly calendar view with completions
- `export <filename.csv>` - Export all data to CSV
- `remind <habit_name> <HH:MM>` - Set reminder time (24-hour format)
- `help` - Show available commands
- `quit` - Exit the program

### Example Session

```
🎯 Welcome to Habit Tracker v2.0!
Type 'help' for commands

> add Workout 4 weekly
✅ Added habit 'Workout'

> add Meditation daily
✅ Added habit 'Meditation'

> add Reading 20 monthly
✅ Added habit 'Reading'

> category Workout Fitness
✅ Set category 'Fitness' for 'Workout'

> category Meditation Wellness
✅ Set category 'Wellness' for 'Meditation'

> remind Meditation 07:00
✅ Set reminder for 'Meditation' at 07:00

> done Workout
✅ Marked 'Workout' as done for today! 🎉

> done Meditation
✅ Marked 'Meditation' as done for today! 🎉

> list

📊 YOUR HABITS
======================================================================

📁 Fitness

  [✓] Workout [4/week] (🔥 1 day | 🏆 best: 1)
      Week:  ██░░░░░░░░ 1/4  ○ 1/4
      Month: 1 completions

📁 Wellness

  [✓] Meditation [Daily] (🔥 1 day | 🏆 best: 1)
      Week:  ██░░░░░░░░ 1/7  ○ 1/7
      Month: 1 completions
      ⏰ Reminder: 07:00

📁 Uncategorized

  [ ] Reading [20/month] (🔥 0 days | 🏆 best: 0)
      Week:  ░░░░░░░░░░ 0/7  ○ 0/7
      Month: 0 completions

> stats Workout

📈 STATS FOR: Workout
==================================================
🎯 Goal: 4/week
🔥 Current streak: 1 days
🏆 Longest streak: 1 days
📅 Created: 2025-01-31
✅ Total completions: 1
📊 Last 7 days: 1
📊 Last 30 days: 1
📁 Category: Fitness
📈 This week completion: 25%

Recent completions:
  • 2025-01-31 (Friday)

> calendar Meditation

📅 CALENDAR FOR: Meditation

  January 2025
  Mo Tu We Th Fr Sa Su
                 1  2  3  4
   5  6  7  8  9 10 11
  12 13 14 15 16 17 18
  19 20 21 22 23 24 25
  26 27 28 29 30 31

  ● Days with completion shown in green

> export my_habits.csv
✅ Exported to 'my_habits.csv'

> categories

📁 CATEGORIES:
  • Fitness (1 habit)
  • Wellness (1 habit)
```

## Rust Concepts Demonstrated

### Structs & Enums
- `Habit` struct to represent individual habits
- `HabitTracker` struct for managing the collection
- `GoalType` enum for different goal types (Daily, Weekly, Monthly)

### File I/O
- JSON serialization/deserialization with `serde`
- CSV writing with the `csv` crate
- Persistent storage in `habits.json`

### Time Handling
- Uses `chrono` for date operations
- Streak calculation algorithms
- Weekly/monthly statistics
- Calendar generation

### External Crates
- `colored` - Terminal color output
- `csv` - CSV export functionality
- `notify-rust` - Desktop notifications (Linux/Mac)
- `serde` & `serde_json` - Data serialization

### State Management
- HashMap for storing habits
- Load/save functionality
- Mutable state updates

### CLI Interaction
- Interactive command loop
- User input parsing
- Pretty-printed output with colors and emojis

### Additional Rust Features
- Pattern matching for command parsing
- Option and Result types for error handling
- Trait implementations
- Conditional compilation (`#[cfg]` for platform-specific code)
- Iterator chains and functional programming

## New Features Explained

### 🎯 Custom Goals
Set realistic goals for each habit:
- **Daily**: Every day of the week (7/7)
- **Weekly**: Specific number per week (e.g., "4 weekly" = 4 times per week)
- **Monthly**: Specific number per month (e.g., "20 monthly")

### 📁 Categories
Organize your habits into categories like:
- Fitness (Workout, Running, Yoga)
- Learning (Reading, Coding, Language)
- Wellness (Meditation, Sleep, Hydration)
- Productivity (Writing, Journaling)

### 🏆 Longest Streak
Track your all-time best performance! The app now shows:
- Current streak (consecutive days from today)
- Longest streak (your personal record)

### 📅 Calendar View
See your habit completions in a monthly calendar format. Completed days are highlighted in green, making it easy to spot patterns and gaps.

### 📤 CSV Export
Export all your habit data to CSV for:
- Backup purposes
- Analysis in Excel/Google Sheets
- Data visualization
- Sharing with accountability partners

### ⏰ Reminders
Set specific times to be reminded about your habits. On Linux and Mac, you'll get desktop notifications. On Windows, reminders are shown in the terminal.

### 🎨 Color Coding
- **Green**: Success, goals met, high completion
- **Yellow**: Warnings, approaching goals
- **Red**: Alerts, low completion, goals not met
- **Cyan/Magenta**: Information and categories

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

## License

MIT
