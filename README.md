
# Catmodoro Timer

The Catmodoro Timer is a command-line based productivity tool built in Rust, inspired by the Pomodoro Technique. It aids in managing work sessions with set intervals, providing users with a simple interface to focus on time management effectively. The application includes features for pausing and resuming the timer, as well as the option to exit at any stage of the session.

## Features

- Customizable session durations
- Terminal-based countdown timer
- Pause and resume functionality
- Ability to exit the timer at any point
- Visual, color-coded timer status

## Installation

To use the Catmodoro Timer, you must have Rust and Cargo installed on your system. For installation instructions, please refer to the [Rust official installation guide](https://www.rust-lang.org/tools/install).

Follow these steps to install the Catmodoro Timer:

1. Clone the repository:
   ```bash
   git clone https://github.com/josesustaita/catmodoro.git
   cd catmodoro-timer
   ```

2. Compile the project using Cargo:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be located in `target/release`. You can execute it from there or add it to your path for convenience.

## Usage

Run the timer with the default setting (25 minutes) by executing the binary:

```bash
./catmodoro_timer
```

For a custom session length, use the `-d` or `--duration` flag with the desired number of minutes:

```bash
./catmodoro_timer -d 15
```

### Controls

- `p`: Pause or resume the session.
- `q` or `Esc`: Quit the timer.

### Exiting

You can exit the Catmodoro Timer anytime by pressing `q` or the `Escape` key.

## Contributions

Your contributions are welcome! If you would like to contribute to the project, please feel free to make a pull request or open an issue on GitHub.

## License

This project is released under the [MIT License](LICENSE).
```

