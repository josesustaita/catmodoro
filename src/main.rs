use clap::{App, Arg};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

struct PomodoroApp {
    total: Duration,
    remaining: Duration,
    last_tick: Instant,
    paused: bool,
    animate: bool,
    frame: u64,
}

impl PomodoroApp {
    fn new(minutes: u64, animate: bool) -> Self {
        let total = Duration::from_secs(minutes * 60);
        Self {
            total,
            remaining: total,
            last_tick: Instant::now(),
            paused: false,
            animate,
            frame: 0,
        }
    }

    fn tick(&mut self, now: Instant) {
        if !self.paused {
            self.remaining = self
                .remaining
                .saturating_sub(now.saturating_duration_since(self.last_tick));
            self.frame = self.frame.wrapping_add(1);
        }
        self.last_tick = now;
    }

    fn toggle_pause(&mut self, now: Instant) {
        self.tick(now);
        self.paused = !self.paused;
    }

    fn restart(&mut self, now: Instant) {
        self.remaining = self.total;
        self.last_tick = now;
        self.paused = false;
        self.frame = 0;
    }

    fn draw<B: Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(2),
                            Constraint::Min(5),
                            Constraint::Length(3),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                let finished = self.remaining.is_zero();
                let cat = if finished {
                    " /\\_/\\\n( ^.^ )\n / > *   purr-fect. take a break!"
                } else if self.paused {
                    " /\\_/\\\n( -.- )\n / > z   holding your place"
                } else if self.animate && self.frame / 4 % 2 == 1 {
                    " /\\_/\\\n( o.o )\n / > /   one paw at a time"
                } else {
                    " /\\_/\\\n( o.o )\n / > \\   one paw at a time"
                };
                let color = if finished {
                    Color::LightGreen
                } else if self.paused {
                    Color::Yellow
                } else {
                    Color::LightMagenta
                };
                f.render_widget(
                    Paragraph::new("C A T M O D O R O   /   small steps, sharp focus")
                        .alignment(Alignment::Center)
                        .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                    chunks[0],
                );
                f.render_widget(
                    Paragraph::new(cat)
                        .alignment(Alignment::Center)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" your focus companion "),
                        )
                        .style(Style::default().fg(color)),
                    chunks[1],
                );
                let seconds =
                    self.remaining.as_secs() + u64::from(self.remaining.subsec_nanos() > 0);
                let state = if finished {
                    "COMPLETE"
                } else if self.paused {
                    "PAUSED"
                } else {
                    "FOCUS"
                };
                let label = format!("{}  {:02}:{:02}", state, seconds / 60, seconds % 60);
                f.render_widget(
                    Gauge::default()
                        .block(Block::default().borders(Borders::ALL))
                        .gauge_style(Style::default().fg(color))
                        .ratio(
                            (self.remaining.as_secs_f64() / self.total.as_secs_f64())
                                .clamp(0.0, 1.0),
                        )
                        .label(label),
                    chunks[2],
                );
                f.render_widget(
                    Paragraph::new(
                        "[p / space] pause   [r] restart   [a] animation   [q / esc] quit",
                    )
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Gray)),
                    chunks[3],
                );
            })
            .map(|_| ())
    }
}

// Drop also runs during unwinding, so terminal state survives errors and panics.
struct TerminalSession;
impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
    }
}

fn run(minutes: u64, animate: bool) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let _session = TerminalSession;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.hide_cursor()?;
    let mut app = PomodoroApp::new(minutes, animate);
    loop {
        app.tick(Instant::now());
        app.draw(&mut terminal)?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Char('p') | KeyCode::Char(' ') => app.toggle_pause(Instant::now()),
                    KeyCode::Char('r') => app.restart(Instant::now()),
                    KeyCode::Char('a') => app.animate = !app.animate,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Catmodoro")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jose Sustaita")
        .about("A tiny terminal cat to keep your focus company")
        .arg(
            Arg::new("duration")
                .short('d')
                .long("duration")
                .value_name("MINUTES")
                .default_value("25")
                .takes_value(true)
                .validator(|value| match value.parse::<u64>() {
                    Ok(1..=1440) => Ok(()),
                    _ => Err("choose a whole number between 1 and 1440"),
                })
                .help("Focus session length (1–1440 minutes)"),
        )
        .arg(
            Arg::new("no-animation")
                .long("no-animation")
                .help("Keep your cat still"),
        )
        .get_matches();
    let minutes = matches.value_of("duration").unwrap().parse()?;
    run(minutes, !matches.is_present("no-animation"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_saturates_instead_of_underflowing() {
        let mut app = PomodoroApp::new(1, true);
        app.tick(app.last_tick + Duration::from_secs(61));
        assert_eq!(app.remaining, Duration::ZERO);
        app.tick(app.last_tick + Duration::from_secs(10));
        assert_eq!(app.remaining, Duration::ZERO);
    }

    #[test]
    fn pause_excludes_elapsed_time_and_freezes_animation() {
        let mut app = PomodoroApp::new(1, true);
        app.toggle_pause(app.last_tick + Duration::from_secs(10));
        let frame = app.frame;
        app.tick(app.last_tick + Duration::from_secs(20));
        assert_eq!(app.remaining, Duration::from_secs(50));
        assert_eq!(app.frame, frame);
        app.toggle_pause(app.last_tick);
        app.tick(app.last_tick + Duration::from_secs(5));
        assert_eq!(app.remaining, Duration::from_secs(45));
    }

    #[test]
    fn restart_restores_a_full_active_session() {
        let mut app = PomodoroApp::new(1, false);
        app.toggle_pause(app.last_tick + Duration::from_secs(20));
        app.restart(app.last_tick);
        assert_eq!(app.remaining, Duration::from_secs(60));
        assert!(!app.paused);
    }

    #[test]
    fn completion_can_draw_on_a_small_terminal() {
        let mut app = PomodoroApp::new(1, false);
        app.tick(app.last_tick + Duration::from_secs(60));
        let mut terminal = Terminal::new(tui::backend::TestBackend::new(30, 12)).unwrap();
        app.draw(&mut terminal).unwrap();
    }
}
