# 🎯 Feature Showcase - Habit Tracker v2.0

This document provides a detailed overview of all the new features added to the habit tracker.

## 📊 Feature Matrix

| Feature | Status | Description |
|---------|--------|-------------|
| Custom Goals | ✅ | Daily, weekly (N/week), monthly (N/month) targets |
| Categories/Tags | ✅ | Organize habits by category |
| CSV Export | ✅ | Export all data for external analysis |
| Notifications | ✅ | Time-based reminders (Linux/Mac) |
| Color Output | ✅ | Full color-coded terminal interface |
| Calendar View | ✅ | Monthly calendar with completion visualization |
| Longest Streak | ✅ | Track all-time best performance |

## 🎯 Custom Goals in Detail

### Goal Types

#### Daily Goals
```
> add Exercise daily
```
- Target: 7 completions per week
- Perfect for: Essential daily habits like medication, hydration, etc.

#### Weekly Goals
```
> add Gym 3 weekly
> add Reading 5 weekly
```
- Target: Specific number of completions per week
- Perfect for: Flexible habits that don't need daily completion
- Examples: Gym 3x/week, reading 5x/week, meal prep 2x/week

#### Monthly Goals
```
> add Social Event 8 monthly
> add Deep Work 20 monthly
```
- Target: Specific number of completions per month
- Perfect for: Less frequent but important activities
- Examples: Social events, creative projects, networking

### Goal Progress Visualization

The app shows your progress with color-coded indicators:
- **Green ✓ Goal Met!** - You've hit your target
- **Yellow ⚠ 3/4** - You're 70%+ toward your goal
- **Red ○ 1/4** - Below 70% completion

## 📁 Categories & Organization

### Setting Categories
```
> category Workout Fitness
> category Reading Learning
> category Meditation Wellness
```

### Benefits
- **Visual Organization**: Habits grouped by category in list view
- **Filtering**: `list Fitness` shows only Fitness habits
- **Statistics**: See how many habits in each category
- **Mental Clarity**: Better understanding of habit distribution

### Category Ideas
- **Health**: Exercise, Sleep, Nutrition, Hydration
- **Learning**: Reading, Courses, Practice, Study
- **Creativity**: Writing, Art, Music, Projects
- **Social**: Calls, Events, Networking
- **Productivity**: Deep Work, Planning, Review
- **Wellness**: Meditation, Journaling, Therapy

## 📤 CSV Export

### Usage
```
> export my_habits.csv
> export backup_2025_01.csv
```

### CSV Format
The export includes:
- Habit Name
- Category
- Goal Type
- Created Date
- Current Streak
- Longest Streak
- Total Completions
- Weekly Count
- Monthly Count
- Completion Rate (%)

### Use Cases
1. **Backup**: Regular exports for data safety
2. **Analysis**: Import into Excel/Google Sheets for deeper insights
3. **Visualization**: Create charts and graphs
4. **Sharing**: Share progress with accountability partners
5. **Migration**: Move data to other systems

## ⏰ Reminder Notifications

### Setting Reminders
```
> remind Meditation 07:00
> remind Workout 18:30
> remind Journaling 21:00
```

### Features
- 24-hour time format (HH:MM)
- Desktop notifications (Linux/Mac via notify-rust)
- Only reminds if habit not completed today
- Automatic check on app startup

### Platform Support
- **Linux/Mac**: Full desktop notifications with system integration
- **Windows**: Terminal-based reminder messages

### Best Practices
- Set reminders for habits you want to do at specific times
- Use morning reminders for habits best done early
- Evening reminders for reflection habits
- Multiple reminders throughout the day for frequent habits

## 🎨 Color-Coded Output

### Color Scheme

#### Status Colors
- **Green**: Success messages, completed habits, goals met
- **Yellow**: Warnings, approaching goals, reminders
- **Red**: Errors, low completion, goals not met
- **Cyan**: Headers, information
- **Magenta**: Categories
- **White/Bold**: Emphasis on important text

#### Visual Indicators
```
[✓] Completed today      (Green checkmark)
[ ] Not completed        (Empty space)
🔥 Current streak        (Fire emoji)
🏆 Longest streak        (Trophy emoji)
⏰ Reminder set          (Alarm clock)
📁 Category              (Folder icon)
```

### Progress Bars
- **Green bars**: Goal met (100%+)
- **Yellow bars**: Good progress (70-99%)
- **Red bars**: Needs attention (<70%)

## 📅 Calendar View

### Usage
```
> calendar Workout
> calendar Reading
```

### Features
- Shows current month by default
- Completed days highlighted in green
- Standard calendar format (Mo-Su)
- Quick visual pattern recognition

### Benefits
1. **Pattern Recognition**: Spot consistency or gaps at a glance
2. **Motivation**: See your progress visually
3. **Planning**: Identify which days to focus on
4. **Reflection**: Review monthly performance

### Example Output
```
📅 CALENDAR FOR: Workout

  January 2025
  Mo Tu We Th Fr Sa Su
         1  2  3  4  5
   6  7  8  9 10 11 12
  13 14 15 16 17 18 19
  20 21 22 23 24 25 26
  27 28 29 30 31

  ● Days with completion shown in green
```

## 🏆 Longest Streak Tracking

### Why It Matters
- **Motivation**: See your personal best
- **Goals**: Aim to beat your record
- **Perspective**: Even if current streak breaks, you have history
- **Resilience**: Don't feel discouraged by a broken streak

### Display
Every habit shows:
```
[✓] Workout (🔥 15 days | 🏆 best: 23)
```
- Current streak: 15 days
- Longest ever: 23 days

### Strategy
1. Start small and build consistency
2. Don't break the chain
3. If you miss a day, restart without guilt
4. Aim to beat your personal best
5. Track multiple streaks across different habits

## 🎓 Learning from the Code

### New Rust Patterns

#### Enum with Associated Data
```rust
enum GoalType {
    Daily,
    Weekly(u32),
    Monthly(u32),
}
```

#### Platform-Specific Code
```rust
#[cfg(not(target_os = "windows"))]
fn show_notification(&self, title: &str, body: &str) {
    // Linux/Mac implementation
}

#[cfg(target_os = "windows")]
fn show_notification(&self, _title: &str, _body: &str) {
    // Windows implementation
}
```

#### Calendar Algorithm
The calendar generation demonstrates:
- Date arithmetic with chrono
- Weekday calculations
- Loop control
- String formatting
- Conditional coloring

#### Streak Calculation
Shows advanced iterator usage:
- Sliding windows
- Date comparisons
- State tracking
- Edge case handling

## 📈 Performance & Scaling

### Current Implementation
- All data stored in memory
- JSON file for persistence
- O(n) operations for most commands
- Suitable for 100s of habits

### If You Need More
For 1000+ habits or years of data:
1. Consider SQLite database
2. Implement pagination
3. Add indexing
4. Lazy loading of old data

## 🔒 Privacy & Data

### Data Storage
- All data stored locally in `habits.json`
- No cloud sync (privacy-first)
- No telemetry or tracking
- You own your data completely

### Backup Strategy
1. Regular CSV exports
2. Git commit `habits.json`
3. Cloud backup of JSON file
4. Multiple device copies

## 🚀 Quick Tips

1. **Start with 3-5 habits max** - Don't overwhelm yourself
2. **Use categories early** - Easier than reorganizing later
3. **Set realistic goals** - Better to exceed a modest goal than fail an ambitious one
4. **Export weekly** - Regular backups prevent data loss
5. **Review the calendar** - Visual patterns reveal insights
6. **Celebrate longest streaks** - Your achievements matter
7. **Use reminders strategically** - Not for every habit, just the ones you forget
8. **Check stats regularly** - Data motivates

## 🎯 Example Workflows

### Morning Routine
```
> list Wellness
> done Meditation
> done Exercise
> done Journaling
```

### Weekly Review
```
> categories
> stats Workout
> stats Reading
> export weekly_review.csv
```

### Setup New Habit
```
> add "Learn Rust" 5 weekly
> category "Learn Rust" Learning
> remind "Learn Rust" 19:00
```

### Monthly Reflection
```
> calendar Meditation
> calendar Workout
> stats Meditation
> export habits_january_2025.csv
```

Happy habit tracking! 🎉
