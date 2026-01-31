#!/bin/bash

# Example usage demonstration for the Habit Tracker
# (This is a simulation since we can't run Rust in this environment)

echo "🎯 Welcome to Habit Tracker!"
echo "Type 'help' for commands"
echo ""

# Simulating adding habits
echo "> add Workout"
echo "✅ Added habit 'Workout'"
echo ""

echo "> add Meditation"
echo "✅ Added habit 'Meditation'"
echo ""

echo "> add Reading"
echo "✅ Added habit 'Reading'"
echo ""

# Marking some as done
echo "> done Workout"
echo "✅ Marked 'Workout' as done for today! 🎉"
echo ""

echo "> done Reading"
echo "✅ Marked 'Reading' as done for today! 🎉"
echo ""

# Listing all habits
echo "> list"
echo ""
echo "📊 YOUR HABITS"
echo "============================================================"
echo ""
echo "[✓] Workout (🔥 1 day streak)"
echo "    Week:  ██░░░░░░░░ 1/7"
echo "    Month: 1 completions"
echo ""
echo "[✓] Reading (🔥 1 day streak)"
echo "    Week:  ██░░░░░░░░ 1/7"
echo "    Month: 1 completions"
echo ""
echo "[ ] Meditation (🔥 0 day streak)"
echo "    Week:  ░░░░░░░░░░ 0/7"
echo "    Month: 0 completions"
echo ""

# Showing detailed stats
echo "> stats Workout"
echo ""
echo "📈 STATS FOR: Workout"
echo "========================================"
echo "🔥 Current streak: 1 days"
echo "📅 Created: 2025-01-31"
echo "✅ Total completions: 1"
echo "📊 Last 7 days: 1"
echo "📊 Last 30 days: 1"
echo ""
echo "Recent completions:"
echo "  • 2025-01-31"
echo ""

echo "> quit"
echo "👋 Keep building those habits! Goodbye!"
