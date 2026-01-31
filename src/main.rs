use chrono::{Local, NaiveDate, Datelike, Weekday};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum GoalType {
    Daily,
    Weekly(u32), // e.g., 4 times per week
    Monthly(u32), // e.g., 20 times per month
}

impl GoalType {
    fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.as_slice() {
            ["daily"] => Some(GoalType::Daily),
            [n, "per", "week"] | [n, "weekly"] => {
                n.parse::<u32>().ok().map(GoalType::Weekly)
            }
            [n, "per", "month"] | [n, "monthly"] => {
                n.parse::<u32>().ok().map(GoalType::Monthly)
            }
            _ => None,
        }
    }

    fn display(&self) -> String {
        match self {
            GoalType::Daily => "Daily".to_string(),
            GoalType::Weekly(n) => format!("{}/week", n),
            GoalType::Monthly(n) => format!("{}/month", n),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Habit {
    name: String,
    created_date: NaiveDate,
    completions: Vec<NaiveDate>,
    goal: GoalType,
    category: Option<String>,
    reminder_time: Option<String>, // HH:MM format
}

impl Habit {
    fn new(name: String, goal: GoalType, category: Option<String>) -> Self {
        Habit {
            name,
            created_date: Local::now().naive_local().date(),
            completions: Vec::new(),
            goal,
            category,
            reminder_time: None,
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
                check_date = match check_date.pred_opt() {
                    Some(d) => d,
                    None => break,
                };
            } else {
                if check_date == today && streak == 0 {
                    check_date = match check_date.pred_opt() {
                        Some(d) => d,
                        None => break,
                    };
                    if self.completions.contains(&check_date) {
                        streak += 1;
                        check_date = match check_date.pred_opt() {
                            Some(d) => d,
                            None => break,
                        };
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

    fn longest_streak(&self) -> u32 {
        if self.completions.is_empty() {
            return 0;
        }

        let mut max_streak = 1;
        let mut current_streak = 1;

        for i in 1..self.completions.len() {
            let diff = self.completions[i]
                .signed_duration_since(self.completions[i - 1])
                .num_days();

            if diff == 1 {
                current_streak += 1;
                max_streak = max_streak.max(current_streak);
            } else {
                current_streak = 1;
            }
        }

        max_streak
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

    fn goal_progress_this_week(&self) -> (usize, usize) {
        let count = self.weekly_count();
        let target = match self.goal {
            GoalType::Daily => 7,
            GoalType::Weekly(n) => n as usize,
            GoalType::Monthly(_) => 7,
        };
        (count, target)
    }

    fn goal_status(&self) -> String {
        let (current, target) = self.goal_progress_this_week();
        let percentage = (current as f32 / target as f32 * 100.0) as u32;

        if current >= target {
            "✓ Goal Met!".green().to_string()
        } else if percentage >= 70 {
            format!("⚠ {}/{}", current, target).yellow().to_string()
        } else {
            format!("○ {}/{}", current, target).red().to_string()
        }
    }

    fn progress_bar(&self, total: usize) -> String {
        let completed = self.weekly_count();
        let filled = (completed * 10) / total.max(1);
        let empty = 10 - filled;

        let bar = format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(empty)
        );

        if completed >= total {
            bar.green().to_string()
        } else if completed >= (total * 7 / 10) {
            bar.yellow().to_string()
        } else {
            bar.red().to_string()
        }
    }

    fn get_calendar_month(&self, year: i32, month: u32) -> String {
        use chrono::NaiveDate;

        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let days_in_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
        }.signed_duration_since(first_day).num_days();

        let mut output = String::new();
        output.push_str(&format!("\n  {} {}\n",
            first_day.format("%B"),
            year
        ));
        output.push_str("  Mo Tu We Th Fr Sa Su\n  ");

        let start_weekday = first_day.weekday().num_days_from_monday();
        for _ in 0..start_weekday {
            output.push_str("   ");
        }

        for day in 1..=days_in_month {
            let date = NaiveDate::from_ymd_opt(year, month, day as u32).unwrap();
            let day_str = if self.completions.contains(&date) {
                format!("{:2}", day).green().bold().to_string()
            } else {
                format!("{:2}", day)
            };

            output.push_str(&format!("{} ", day_str));

            if date.weekday() == Weekday::Sun {
                output.push_str("\n  ");
            }
        }
        output.push('\n');
        output
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

    fn add_habit(&mut self, name: String, goal: GoalType, category: Option<String>) {
        if self.habits.contains_key(&name) {
            println!("{}", format!("❌ Habit '{}' already exists!", name).red());
        } else {
            self.habits.insert(name.clone(), Habit::new(name.clone(), goal, category));
            println!("{}", format!("✅ Added habit '{}'", name).green());
        }
    }

    fn remove_habit(&mut self, name: &str) {
        if self.habits.remove(name).is_some() {
            println!("{}", format!("✅ Removed habit '{}'", name).green());
        } else {
            println!("{}", format!("❌ Habit '{}' not found!", name).red());
        }
    }

    fn mark_done(&mut self, name: &str) {
        if let Some(habit) = self.habits.get_mut(name) {
            let today = Local::now().naive_local().date();
            if habit.mark_complete(today) {
                println!("{}", format!("✅ Marked '{}' as done for today! 🎉", name).green().bold());

                let (current, target) = habit.goal_progress_this_week();
                if current == target {
                    println!("{}", "🎯 Weekly goal achieved! Great job!".yellow().bold());
                }
            } else {
                println!("{}", format!("ℹ️  '{}' was already marked done today.", name).cyan());
            }
        } else {
            println!("{}", format!("❌ Habit '{}' not found!", name).red());
        }
    }

    fn set_category(&mut self, name: &str, category: String) {
        if let Some(habit) = self.habits.get_mut(name) {
            habit.category = Some(category.clone());
            println!("{}", format!("✅ Set category '{}' for '{}'", category, name).green());
        } else {
            println!("{}", format!("❌ Habit '{}' not found!", name).red());
        }
    }

    fn set_reminder(&mut self, name: &str, time: String) {
        if let Some(habit) = self.habits.get_mut(name) {
            habit.reminder_time = Some(time.clone());
            println!("{}", format!("✅ Set reminder for '{}' at {}", name, time).green());
            #[cfg(not(target_os = "windows"))]
            self.show_notification(
                "Reminder Set",
                &format!("You'll be reminded about '{}' at {}", name, time),
            );
        } else {
            println!("{}", format!("❌ Habit '{}' not found!", name).red());
        }
    }

    fn show_habits(&self, filter_category: Option<&str>) {
        let filtered_habits: Vec<_> = self.habits.values()
            .filter(|h| {
                filter_category.map_or(true, |cat| {
                    h.category.as_ref().map_or(false, |c| c == cat)
                })
            })
            .collect();

        if filtered_habits.is_empty() {
            if let Some(cat) = filter_category {
                println!("\n{}", format!("📋 No habits in category '{}'", cat).yellow());
            } else {
                println!("\n{}", "📋 No habits tracked yet. Add one with 'add <habit_name>'".yellow());
            }
            return;
        }

        println!("\n{}", "📊 YOUR HABITS".bright_cyan().bold());
        println!("{}", "=".repeat(70).bright_black());

        let mut by_category: HashMap<String, Vec<&Habit>> = HashMap::new();
        for habit in &filtered_habits {
            let cat = habit.category.clone().unwrap_or_else(|| "Uncategorized".to_string());
            by_category.entry(cat).or_insert_with(Vec::new).push(habit);
        }

        let mut categories: Vec<_> = by_category.keys().collect();
        categories.sort();

        for category in categories {
            println!("\n{}", format!("📁 {}", category).bright_magenta().bold());

            let mut habits = by_category[category].clone();
            habits.sort_by(|a, b| b.current_streak().cmp(&a.current_streak()));

            for habit in habits {
                let done_today = if habit.is_done_today() { "✓".green() } else { " ".normal() };
                let streak = habit.current_streak();
                let longest = habit.longest_streak();
                let (weekly, target) = habit.goal_progress_this_week();
                let progress = habit.progress_bar(target);
                let status = habit.goal_status();

                println!(
                    "\n  [{}] {} {} (🔥 {} day{} | 🏆 best: {})",
                    done_today,
                    habit.name.bright_white().bold(),
                    format!("[{}]", habit.goal.display()).bright_black(),
                    streak,
                    if streak == 1 { "" } else { "s" },
                    longest
                );
                println!("      Week:  {} {}/{}  {}", progress, weekly, target, status);
                println!("      Month: {} completions", habit.monthly_count());

                if let Some(ref time) = habit.reminder_time {
                    println!("      ⏰ Reminder: {}", time.bright_yellow());
                }
            }
        }
        println!();
    }

    fn show_stats(&self, name: &str) {
        if let Some(habit) = self.habits.get(name) {
            println!("\n{}", format!("📈 STATS FOR: {}", habit.name).bright_cyan().bold());
            println!("{}", "=".repeat(50).bright_black());
            println!("{} {}", "🎯 Goal:".bold(), habit.goal.display());
            println!("{} {}", "🔥 Current streak:".bold(), format!("{} days", habit.current_streak()).bright_yellow());
            println!("{} {}", "🏆 Longest streak:".bold(), format!("{} days", habit.longest_streak()).bright_green());
            println!("{} {}", "📅 Created:".bold(), habit.created_date);
            println!("{} {}", "✅ Total completions:".bold(), habit.completions.len());
            println!("{} {}", "📊 Last 7 days:".bold(), habit.weekly_count());
            println!("{} {}", "📊 Last 30 days:".bold(), habit.monthly_count());

            if let Some(ref cat) = habit.category {
                println!("{} {}", "📁 Category:".bold(), cat.bright_magenta());
            }

            if let Some(ref time) = habit.reminder_time {
                println!("{} {}", "⏰ Reminder:".bold(), time.bright_yellow());
            }

            let (current, target) = habit.goal_progress_this_week();
            let completion_rate = (current as f32 / target as f32 * 100.0) as u32;
            println!("{} {}%", "📈 This week completion:".bold(),
                if completion_rate >= 100 { format!("{}", completion_rate).green() }
                else if completion_rate >= 70 { format!("{}", completion_rate).yellow() }
                else { format!("{}", completion_rate).red() }
            );

            if !habit.completions.is_empty() {
                println!("\n{}", "Recent completions:".bold());
                for date in habit.completions.iter().rev().take(10) {
                    println!("  • {}", date.format("%Y-%m-%d (%A)").to_string().bright_white());
                }
            }
            println!();
        } else {
            println!("{}", format!("❌ Habit '{}' not found!", name).red());
        }
    }

    fn show_calendar(&self, name: &str) {
        if let Some(habit) = self.habits.get(name) {
            let today = Local::now().naive_local().date();
            println!("\n{}", format!("📅 CALENDAR FOR: {}", habit.name).bright_cyan().bold());
            println!("{}", habit.get_calendar_month(today.year(), today.month()));
            println!("{}", "  ● Days with completion shown in green".bright_black());
        } else {
            println!("{}", format!("❌ Habit '{}' not found!", name).red());
        }
    }

    fn export_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;

        wtr.write_record(&[
            "Habit Name",
            "Category",
            "Goal",
            "Created Date",
            "Current Streak",
            "Longest Streak",
            "Total Completions",
            "Weekly Count",
            "Monthly Count",
            "Completion Rate (%)",
        ])?;

        for habit in self.habits.values() {
            let (current, target) = habit.goal_progress_this_week();
            let rate = (current as f32 / target as f32 * 100.0) as u32;

            wtr.write_record(&[
                &habit.name,
                &habit.category.clone().unwrap_or_else(|| "None".to_string()),
                &habit.goal.display(),
                &habit.created_date.to_string(),
                &habit.current_streak().to_string(),
                &habit.longest_streak().to_string(),
                &habit.completions.len().to_string(),
                &habit.weekly_count().to_string(),
                &habit.monthly_count().to_string(),
                &rate.to_string(),
            ])?;
        }

        wtr.flush()?;
        println!("{}", format!("✅ Exported to '{}'", filename).green().bold());
        Ok(())
    }

    fn list_categories(&self) {
        let mut categories: Vec<String> = self.habits.values()
            .filter_map(|h| h.category.clone())
            .collect();
        categories.sort();
        categories.dedup();

        if categories.is_empty() {
            println!("{}", "📁 No categories defined yet.".yellow());
        } else {
            println!("\n{}", "📁 CATEGORIES:".bright_magenta().bold());
            for cat in categories {
                let count = self.habits.values()
                    .filter(|h| h.category.as_ref() == Some(&cat))
                    .count();
                println!("  • {} ({} habit{})", cat.bright_white(), count, if count == 1 { "" } else { "s" });
            }
            println!();
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn show_notification(&self, title: &str, body: &str) {
        use notify_rust::Notification;
        let _ = Notification::new()
            .summary(title)
            .body(body)
            .timeout(5000)
            .show();
    }

    #[cfg(target_os = "windows")]
    fn show_notification(&self, _title: &str, _body: &str) {
        println!("📢 Notification: {} - {}", _title, _body);
    }

    fn check_reminders(&self) {
        let now = Local::now();
        let current_time = now.format("%H:%M").to_string();

        for habit in self.habits.values() {
            if let Some(ref reminder) = habit.reminder_time {
                if reminder == &current_time && !habit.is_done_today() {
                    #[cfg(not(target_os = "windows"))]
                    self.show_notification(
                        "Habit Reminder",
                        &format!("Time to complete: {}", habit.name),
                    );
                    println!("{}",
                        format!("⏰ Reminder: Time to complete '{}'!", habit.name)
                        .bright_yellow().bold()
                    );
                }
            }
        }
    }
}

fn print_help() {
    println!("\n{}", "🎯 HABIT TRACKER COMMANDS".bright_cyan().bold());
    println!("{}", "=".repeat(60).bright_black());
    println!("{}", "  BASIC COMMANDS:".bright_white().bold());
    println!("  add <n> [goal]     - Add habit (goal: daily, 4 weekly, 20 monthly)");
    println!("  remove <n>         - Remove a habit");
    println!("  done <n>           - Mark habit as done today");
    println!("  list [category]       - Show all habits (optionally filter by category)");
    println!("  stats <n>          - Show detailed stats");
    println!();
    println!("{}", "  ADVANCED FEATURES:".bright_white().bold());
    println!("  category <n> <cat> - Set category for a habit");
    println!("  categories            - List all categories");
    println!("  calendar <n>       - Show monthly calendar view");
    println!("  export <file.csv>     - Export all data to CSV");
    println!("  remind <n> <HH:MM> - Set reminder time (24h format)");
    println!();
    println!("{}", "  OTHER:".bright_white().bold());
    println!("  help                  - Show this help");
    println!("  quit                  - Exit the program");
    println!();
    println!("{}", "  EXAMPLES:".bright_yellow());
    println!("  add Workout 4 weekly");
    println!("  category Workout Fitness");
    println!("  remind Workout 07:00");
    println!("  export habits_backup.csv");
    println!();
}

fn main() {
    let mut tracker = HabitTracker::load();

    println!("\n{}", "🎯 Welcome to Habit Tracker v2.0!".bright_cyan().bold());
    println!("{}", "Type 'help' for commands\n".bright_black());

    tracker.check_reminders();

    loop {
        print!("{} ", ">".bright_green());
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
                if parts.len() >= 2 {
                    let mut name_parts = vec![];
                    let mut goal = GoalType::Daily;
                    let mut i = 1;

                    while i < parts.len() {
                        if let Ok(num) = parts[i].parse::<u32>() {
                            if i + 1 < parts.len() {
                                match parts[i + 1].to_lowercase().as_str() {
                                    "weekly" | "week" => {
                                        goal = GoalType::Weekly(num);
                                        break;
                                    }
                                    "monthly" | "month" => {
                                        goal = GoalType::Monthly(num);
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        } else if parts[i].to_lowercase() == "daily" {
                            goal = GoalType::Daily;
                            break;
                        }
                        name_parts.push(parts[i]);
                        i += 1;
                    }

                    let name = name_parts.join(" ");
                    tracker.add_habit(name, goal, None);
                    tracker.save().unwrap();
                } else {
                    println!("{}", "Usage: add <habit_name> [goal]".yellow());
                    println!("{}", "Examples: add Workout, add Reading 4 weekly".bright_black());
                }
            }
            "remove" | "rm" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.remove_habit(&name);
                    tracker.save().unwrap();
                } else {
                    println!("{}", "Usage: remove <habit_name>".yellow());
                }
            }
            "done" | "complete" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.mark_done(&name);
                    tracker.save().unwrap();
                } else {
                    println!("{}", "Usage: done <habit_name>".yellow());
                }
            }
            "list" | "ls" => {
                let filter = if parts.len() > 1 {
                    Some(parts[1..].join(" "))
                } else {
                    None
                };
                tracker.show_habits(filter.as_deref());
            }
            "stats" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.show_stats(&name);
                } else {
                    println!("{}", "Usage: stats <habit_name>".yellow());
                }
            }
            "category" | "cat" => {
                if parts.len() > 2 {
                    let name = parts[1].to_string();
                    let category = parts[2..].join(" ");
                    tracker.set_category(&name, category);
                    tracker.save().unwrap();
                } else {
                    println!("{}", "Usage: category <habit_name> <category>".yellow());
                }
            }
            "categories" | "cats" => {
                tracker.list_categories();
            }
            "calendar" | "cal" => {
                if parts.len() > 1 {
                    let name = parts[1..].join(" ");
                    tracker.show_calendar(&name);
                } else {
                    println!("{}", "Usage: calendar <habit_name>".yellow());
                }
            }
            "export" => {
                let filename = if parts.len() > 1 {
                    parts[1]
                } else {
                    "habits_export.csv"
                };
                match tracker.export_csv(filename) {
                    Ok(_) => {},
                    Err(e) => println!("{}", format!("❌ Export failed: {}", e).red()),
                }
            }
            "remind" | "reminder" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let time = parts[2].to_string();
                    tracker.set_reminder(&name, time);
                    tracker.save().unwrap();
                } else {
                    println!("{}", "Usage: remind <habit_name> <HH:MM>".yellow());
                }
            }
            "check" => {
                tracker.check_reminders();
            }
            "help" | "h" => {
                print_help();
            }
            "quit" | "exit" | "q" => {
                println!("{}", "👋 Keep building those habits! Goodbye!".bright_cyan().bold());
                break;
            }
            _ => {
                println!("{}", format!("❌ Unknown command: '{}'", parts[0]).red());
                println!("{}", "Type 'help' for available commands.".bright_black());
            }
        }
    }
}
