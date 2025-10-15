# Healing-Habits

A TUI (Text User Interface) habit tracker designed for tracking self-care habits on a weekly basis, specifically for PTSD therapy check-ins.

## Features

- **Weekly Tracking**: Track habits across Monday-Sunday cycles
- **Habit Frequencies**: Three frequency types to match your routine
  - **Daily**: Should be done every day (Shower, Brush teeth, Meds)
  - **Weekly**: Should be done once per week (Trim nails)
    - When marked Done on a day, previous days auto-fill as Skipped
    - Example: Mark Done on Tuesday → Monday becomes Skipped, Wed-Sun stay unmarked
  - **As-needed**: Do as needed, no specific frequency
  - All habits show every day - frequency is informational only
- **Non-Destructive Status Cycling**: Cycle through statuses before saving
  - Press Space/Enter repeatedly to find the right status
  - ESC to cancel if you cycled by mistake
  - Changes save automatically when you navigate away
- **Three Status Types**:
  - Done (✓): Habit completed
  - Skipped (✗): Intentionally skipped
  - Unmarked ( ): Not yet tracked
- **Week Strip View**: Visual overview of the entire week with status symbols
- **Daily Details**: See all habits for a selected day with their statuses
- **Notes Support**: Add emotional notes when logging or skipping habits
- **Habit Management**: Add, edit, delete, reorder, and set frequency
- **Weekly Statistics**: View completion rates and trends
- **Export Reports**: Generate markdown reports for therapy check-ins
- **Persistent Storage**: All data saved to JSON automatically
- **Default Habits**: Starts with: Shower (Daily), Brush teeth (Daily), Trim nails (Weekly), Meds (Daily)

## Installation

### Download Pre-built Binaries (Recommended)

Download the latest release for your platform from the [Releases page](https://github.com/davidliedle/Healing-Habits/releases):

- **Linux (x86_64)**: `healing-habits-linux-x86_64.tar.gz`
- **Linux (ARM64)**: `healing-habits-linux-aarch64.tar.gz`
- **macOS (Intel)**: `healing-habits-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `healing-habits-macos-aarch64.tar.gz`
- **Windows (x86_64)**: `healing-habits-windows-x86_64.zip`

Extract the archive and run the binary:

```bash
# Linux/macOS
tar xzf healing-habits-*.tar.gz
chmod +x healing-habits
./healing-habits

# Windows
# Extract the .zip file and run healing-habits.exe
```

### From Source

```bash
# Clone the repository
git clone https://github.com/davidliedle/Healing-Habits.git
cd Healing-Habits

# Build the release binary
cargo build --release
```

## Running the Application

After building, you can run Healing-Habits in several ways:

### Run Directly from Build Directory

```bash
# From the project directory
./target/release/healing-habits
```

### Install to System (Optional)

```bash
# Install to ~/.cargo/bin (ensure it's in your PATH)
cargo install --path .

# Then run from anywhere
healing-habits
```

### Run in Development Mode

```bash
# Run without building release binary (slower, but includes debug info)
cargo run
```

The application will:
1. Create a data directory automatically on first run
2. Initialize with four default habits with smart frequencies:
   - Shower (Daily)
   - Brush teeth (Daily)
   - Trim nails (Weekly - rolls over if not completed)
   - Meds (Daily)
3. Display the main TUI interface with the current week
4. Auto-save all changes to disk

Press `?` at any time to see the help screen with all keyboard shortcuts.

## Usage

### Keyboard Shortcuts

#### Navigation
- `←` / `→` : Move between days
- `↑` / `↓` : Select different habits
- `[` / `]` : Previous/Next week
- `t` : Go to today

#### Actions
- `Space` / `Enter` : Cycle habit status (stages change, doesn't save yet)
- `Esc` : Cancel staged status change
- `n` : Add/edit note for selected habit

**Note**: Status changes save automatically when you navigate to a different day/habit or switch views.

#### Views
- `v` : View weekly statistics
- `h` : Manage habits (add/edit/delete/reorder/set frequency)
- `x` : Export week to markdown
- `?` : Show help screen

#### Habit Management (press 'h')
- `↑` / `↓` : Navigate habits
- `a` : Add new habit
- `e` : Edit habit name
- `d` : Delete habit
- `f` : Cycle frequency (Daily → Weekly → As-needed)
- `[` / `]` : Move habit up/down in list
- `q` / `Esc` : Return to main view

#### Other
- `q` : Quit (saves any staged changes)
- `Ctrl+C` : Quit immediately

## UI Layout

```
┌─────────────────────────────────────────────────────┐
│  Week of Oct 7-13, 2025                             │
├─────────────────────────────────────────────────────┤
│  Mon   Tue   Wed   Thu   Fri   Sat   Sun           │
│  [✓]   [✓]   [✓]   [✗]   [✓]   [ ]   [ ]          │
├─────────────────────────────────────────────────────┤
│  Selected: Thursday, Oct 10                         │
│                                                     │
│  Habits for this day:                              │
│  ► Shower           [Skipped]                      │
│    Brush teeth      [Done]                         │
│    Take meds        [Done]                         │
│                                                     │
│  Note: "PTSD flare-up today, couldn't manage       │
│         shower. Brushed teeth though - small win"  │
│                                                     │
│  [e] Edit note  [v] View stats  [h] Manage habits  │
└─────────────────────────────────────────────────────┘
```

## Data Storage

Data is automatically saved to:
- **macOS**: `~/Library/Application Support/healing-habits/habits.json`
- **Linux**: `~/.local/share/healing-habits/habits.json`
- **Windows**: `%LOCALAPPDATA%\healing-habits\habits.json`

Weekly exports are saved to:
- **All platforms**: `~/Documents/healing-habits-exports/habit-report-YYYY-MM-DD.md`

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

## Roadmap

- [x] Week strip with day status symbols
- [x] Daily habit list with status toggle
- [x] Non-destructive status cycling (stage before save)
- [x] Habit frequencies (Daily, Weekly, As-needed - informational)
- [x] Weekly statistics view
- [x] Persistent JSON storage
- [x] Navigation (days/weeks/habits)
- [x] Note editing functionality
- [x] Habit management (add/remove/edit/reorder/frequency)
- [x] Export weekly report for therapy
- [ ] Multi-week history view
- [ ] Habit streaks and trends
- [ ] Customizable habit categories
- [ ] Configurable week start day

## License

MIT License - see [LICENSE](LICENSE) file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you're using this tool for mental health tracking, please remember:
- This is a supplementary tool, not a replacement for professional help
- Celebrate small wins - every tracked habit is progress
- Be kind to yourself on difficult days

## Author

David Liedle
