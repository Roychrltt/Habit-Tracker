use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Habit {
    name: String,
    created_date: NaiveDate,
    completions: Vec<NaiveDate>,
}

impl Habit {
    fn new(name: String) -> Self {
        Habit {
            name,
            created_date: Local::now().naive_local().date(),
            completions: Vec::new(),
        }
    }

    fn mark_complete(&mut self, date: NaiveDate) -> bool {
        if !self.completions.contains(&date) {
            self.completions.push(date);
            self.completions.sort();
            true
        } else {
            false
        }
    }

    fn current_streak(&self) -> u32 {
        let today = Local::now().naive_local().date();
        let mut streak = 0;
        let mut check_date = today;

        loop {
            if self.completions.contains(&check_date) {
                streak += 1;
                check_date = check_date.pred_opt().unwrap();
            } else {
                // Allow one day gap for "today not done yet"
                if check_date == today && streak == 0 {
                    check_date = check_date.pred_opt().unwrap();
                    if self.completions.contains(&check_date) {
                        streak += 1;
                        check_date = check_date.pred_opt().unwrap();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        streak
    }

    fn weekly_count(&self) -> usize {
        let today = Local::now().naive_local().date();
        let week_ago = today - chrono::Duration::days(7);
        self.completions
            .iter()
            .filter(|&&d| d > week_ago && d <= today)
            .count()
    }

    fn monthly_count(&self) -> usize {
        let today = Local::now().naive_local().date();
        let month_ago = today - chrono::Duration::days(30);
        self.completions
            .iter()
            .filter(|&&d| d > month_ago && d <= today)
            .count()
    }

    fn is_done_today(&self) -> bool {
        let today = Local::now().naive_local().date();
        self.completions.contains(&today)
    }

    fn progress_bar(&self, total: usize) -> String {
        let completed = self.weekly_count();
        let filled = (completed * 10) / total.max(1);
        let empty = 10 - filled;
        
        format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(empty)
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HabitTracker {
    habits: HashMap<String, Habit>,
}

impl HabitTracker {
    fn new() -> Self {
        HabitTracker {
            habits: HashMap::new(),
        }
    }

    fn load() -> Self {
        match fs::read_to_string("habits.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| HabitTracker::new()),
            Err(_) => HabitTracker::new(),
        }
    }

    fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write("habits.json", json)?;
        Ok(())
    }

    fn add_habit(&mut self, name: String) {
        if self.habits.contains_key(&name) {
            println!("❌ Habit '{}' already exists!", name);
        } else {
            self.habits.insert(name.clone(), Habit::new(name.clone()));
            println!("✅ Added habit '{}'", name);
        }
    }

    fn remove_habit(&mut self, name: &str) {
        if self.habits.remove(name).is_some() {
            println!("✅ Removed habit '{}'", name);
        } else {
            println!("❌ Habit '{}' not found!", name);
        }
    }

    fn mark_done(&mut self, name: &str) {
        if let Some(habit) = self.habits.get_mut(name) {
            let today = Local::now().naive_local().date();
            if habit.mark_complete(today) {
                println!("✅ Marked '{}' as done for today! 🎉", name);
            } else {
                println!("ℹ️  '{}' was already marked done today.", name);
            }
        } else {
            println!("❌ Habit '{}' not found!", name);
        }
    }

    fn show_habits(&self) {
        if self.habits.is_empty() {
            println!("\n📋 No habits tracked yet. Add one with 'add <habit_name>'");
            return;
        }

        println!("\n📊 YOUR HABITS");
        println!("{}", "=".repeat(60));

        let mut habits: Vec<_> = self.habits.values().collect();
        habits.sort_by(|a, b| b.current_streak().cmp(&a.current_streak()));

        for habit in habits {
            let done_today = if habit.is_done_today() { "✓" } else { " " };
            let streak = habit.current_streak();
            let weekly = habit.weekly_count();
            let progress = habit.progress_bar(7);

            println!(
                "\n[{}] {} (🔥 {} day streak)",
                done_today, habit.name, streak
            );
            println!("    Week:  {} {}/7", progress, weekly);
            println!("    Month: {} completions", habit.monthly_count());
        }
        println!();
    }

    fn show_stats(&self, name: &str) {
        if let Some(habit) = self.habits.get(name) {
            println!("\n📈 STATS FOR: {}", habit.name);
            println!("{}", "=".repeat(40));
            println!("🔥 Current streak: {} days", habit.current_streak());
            println!("📅 Created: {}", habit.created_date);
            println!("✅ Total completions: {}", habit.completions.len());
            println!("📊 Last 7 days: {}", habit.weekly_count());
            println!("📊 Last 30 days: {}", habit.monthly_count());
            
            if !habit.completions.is_empty() {
                println!("\nRecent completions:");
                for date in habit.completions.iter().rev().take(10) {
                    println!("  • {}", date);
                }
            }
            println!();
        } else {
            println!("❌ Habit '{}' not found!", name);
        }
    }
}

fn print_help() {
    println!("\n🎯 HABIT TRACKER COMMANDS");
    println!("{}", "=".repeat(40));
    println!("  add <name>       - Add a new habit");
    println!("  remove <name>    - Remove a habit");
    println!("  done <name>      - Mark habit as done today");
    println!("  list             - Show all habits");
    println!("  stats <name>     - Show detailed stats");
    println!("  help             - Show this help");
    println!("  quit             - Exit the program");
    println!();
}

fn main() {
    let mut tracker = HabitTracker::load();
    println!("\n🎯 Welcome to Habit Tracker!");
    println!("Type 'help' for commands\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "add" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.add_habit(name);
                    tracker.save().unwrap();
                } else {
                    println!("Usage: add <habit_name>");
                }
            }
            "remove" | "rm" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.remove_habit(&name);
                    tracker.save().unwrap();
                } else {
                    println!("Usage: remove <habit_name>");
                }
            }
            "done" | "complete" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.mark_done(&name);
                    tracker.save().unwrap();
                } else {
                    println!("Usage: done <habit_name>");
                }
            }
            "list" | "ls" => {
                tracker.show_habits();
            }
            "stats" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.show_stats(&name);
                } else {
                    println!("Usage: stats <habit_name>");
                }
            }
            "help" | "h" => {
                print_help();
            }
            "quit" | "exit" | "q" => {
                println!("👋 Keep building those habits! Goodbye!");
                break;
            }
            _ => {
                println!("❌ Unknown command. Type 'help' for available commands.");
            }
        }
    }
}
