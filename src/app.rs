use crate::{
    event::{AppEvent, Event, EventHandler},
    tm::TuringMachine,
};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

pub const RULE_PANE_WIDTH: u16 = 25;

const N_STATES: usize = 4;
const N_SYMBOLS: usize = 4;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub turing_machine_running: bool,
    rng: SmallRng,
    pub tm: TuringMachine<N_STATES, N_SYMBOLS>,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        let mut rng = SmallRng::seed_from_u64(1234);
        Self {
            running: true,
            turing_machine_running: true,
            rng: SmallRng::seed_from_u64(rng.random_range(0..=u64::MAX)),
            tm: TuringMachine::new(16, 32, &mut rng),
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Resize(columns, rows) => {
                    self.tm.resize_memory(rows, columns - RULE_PANE_WIDTH);
                }
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                _ => {}
            },
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('R') | KeyCode::Char('r')) => {
                self.tm.randomize_ruleset(&mut self.rng)
            }
            (_, KeyCode::Char('M') | KeyCode::Char('m')) => self.tm.mutate_ruleset(&mut self.rng),
            (_, KeyCode::Right) => self.tm.double_speed(),
            (_, KeyCode::Left) => self.tm.halve_speed(),
            (_, KeyCode::Backspace) => self.tm.reset_memory(),
            (_, KeyCode::Char('p') | KeyCode::Char('P')) => {
                self.turing_machine_running = !self.turing_machine_running
            }
            _ => {}
        }
        Ok(())
    }

    pub fn tick(&mut self) {
        if self.turing_machine_running {
            self.tm.update();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
