# 🚀 Quick Start Guide

## Getting Started

1. **Install Rust** (if you haven't already):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Navigate to the project**:
   ```bash
   cd habit_tracker
   ```

3. **Run the program**:
   ```bash
   cargo run
   ```

## First Steps

Once the program starts, try these commands:

```
> add Exercise
> add Reading  
> add Meditation
> done Exercise
> list
> stats Exercise
```

## Key Rust Concepts You'll Learn

### 1. **Structs**
```rust
struct Habit {
    name: String,
    created_date: NaiveDate,
    completions: Vec<NaiveDate>,
}
```

### 2. **Enums & Pattern Matching**
```rust
match parts[0].to_lowercase().as_str() {
    "add" => { /* ... */ },
    "done" => { /* ... */ },
    _ => { /* ... */ }
}
```

### 3. **File I/O with Serde**
```rust
fn save(&self) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&self)?;
    fs::write("habits.json", json)?;
    Ok(())
}
```

### 4. **Time Handling with Chrono**
```rust
let today = Local::now().naive_local().date();
let week_ago = today - chrono::Duration::days(7);
```

### 5. **Collections (HashMap)**
```rust
habits: HashMap<String, Habit>
```

### 6. **Error Handling**
```rust
match fs::read_to_string("habits.json") {
    Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| HabitTracker::new()),
    Err(_) => HabitTracker::new(),
}
```

## Code Highlights

### Streak Calculation
The `current_streak()` method walks backwards from today, counting consecutive days:
```rust
fn current_streak(&self) -> u32 {
    let today = Local::now().naive_local().date();
    let mut streak = 0;
    let mut check_date = today;
    
    loop {
        if self.completions.contains(&check_date) {
            streak += 1;
            check_date = check_date.pred_opt().unwrap();
        } else {
            break;
        }
    }
    streak
}
```

### ASCII Progress Bar
```rust
fn progress_bar(&self, total: usize) -> String {
    let completed = self.weekly_count();
    let filled = (completed * 10) / total.max(1);
    let empty = 10 - filled;
    
    format!("{}{}",
        "█".repeat(filled),
        "░".repeat(empty)
    )
}
```

## Building for Release

Create an optimized binary:
```bash
cargo build --release
```

The binary will be at `target/release/habit_tracker` and you can copy it anywhere!

## Extending the Project

Try adding these features to practice more Rust:
- **Colors**: Use the `colored` crate for colorful output
- **Config file**: Support user preferences (TOML/YAML)
- **Custom goals**: Allow "3 times per week" instead of daily
- **Undo command**: Remove today's completion
- **Calendar view**: Show a month calendar with completions
- **Multiple completions per day**: Track how many times per day
- **Export**: Generate CSV reports

Happy coding! 🦀
