use clap::{App as ClapApp, Arg};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io, panic,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Terminal,
};

struct PomodoroApp {
    duration: u64,
    start_time: Instant,
    pause_time: Option<Instant>,
    finished: bool,
    paused: bool,
}

impl PomodoroApp {
    fn new(duration: u64) -> PomodoroApp {
        PomodoroApp {
            duration,
            start_time: Instant::now(),
            pause_time: None,
            finished: false,
            paused: false,
        }
    }

    fn advance(&mut self) {
        if !self.paused
            && Instant::now().duration_since(self.start_time).as_secs() >= self.duration * 60
        {
            self.finished = true;
        }
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        if self.paused {
            self.pause_time = Some(Instant::now());
        } else {
            if let Some(pause_time) = self.pause_time {
                let pause_duration = pause_time.elapsed();
                self.start_time += pause_duration;
            }
            self.pause_time = None;
        }
    }

    fn draw<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Catmodoro Timer")
                .borders(Borders::ALL);
            let elapsed = if self.paused && self.pause_time.is_some() {
                self.pause_time
                    .unwrap()
                    .duration_since(self.start_time)
                    .as_secs()
            } else {
                Instant::now().duration_since(self.start_time).as_secs()
            };
            let remaining_secs = self.duration * 60 - elapsed;

            let percentage = (remaining_secs as f64 / (self.duration * 60) as f64) * 100.0;

            let color = match percentage {
                r if r > 66.0 => Color::LightMagenta,
                r if r > 33.0 => Color::LightCyan,
                _ => Color::LightRed,
            };

            let label = if remaining_secs >= 60 {
                format!("{:02}:{:02}", remaining_secs / 60, remaining_secs % 60)
            } else {
                format!("00:{:02}", remaining_secs)
            };

            let gauge = Gauge::default()
                .block(block)
                .gauge_style(Style::default().fg(color))
                .percent(percentage as u16)
                .label(label);
            f.render_widget(gauge, size);
        })?;

        Ok(())
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut PomodoroApp,
) -> Result<(), Box<dyn Error>> {
    loop {
        app.advance();
        if app.finished {
            break;
        }
        app.draw(terminal)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => app.toggle_pause(),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = ClapApp::new("Rust Catmodoro Timer")
        .version("1.0")
        .author("Your Name")
        .about("Manages a Catmodoro Timer")
        .arg(
            Arg::new("duration")
                .short('d')
                .long("duration")
                .value_name("MINUTES")
                .help("Sets the duration of the Catmodoro in minutes")
                .takes_value(true),
        )
        .get_matches();

    let duration_str = matches.value_of("duration").unwrap_or("25");
    let duration = duration_str
        .parse::<u64>()
        .expect("Failed to parse duration");

    let result = panic::catch_unwind(|| {
        enable_raw_mode().expect("Failed to enable raw mode");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .expect("Failed to enter alternate screen and enable mouse capture");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Failed to create terminal");
        let mut app = PomodoroApp::new(duration);

        let res = run_app(&mut terminal, &mut app);

        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("Failed to leave alternate screen and disable mouse capture");
        terminal.show_cursor().expect("Failed to show cursor");

        if let Err(err) = res {
            println!("Error: {:?}", err);
        }
    });

    if let Err(_) = result {
        disable_raw_mode()?;
        println!("The application panicked, but the terminal was restored.");
    }

    Ok(())
}
