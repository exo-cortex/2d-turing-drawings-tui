use crate::{
    event::{AppEvent, Event, EventHandler},
    turing_machine::TuringMachineInstance,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub turing_machine_running: bool,
    pub turing_machine: TuringMachineInstance,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            turing_machine_running: false,
            turing_machine: TuringMachineInstance::new(54321),
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
                // crossterm::event::Event::Resize(columns, rows) => {}
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
            (_, KeyCode::Char('R') | KeyCode::Char('r')) => self.turing_machine.randomize_ruleset(),
            (_, KeyCode::Char('M') | KeyCode::Char('m')) => self.turing_machine.mutate_ruleset(),
            (_, KeyCode::Right) => self
                .turing_machine
                .change_speed(crate::turing_machine::SpeedUpdate::Increase),
            (_, KeyCode::Left) => self
                .turing_machine
                .change_speed(crate::turing_machine::SpeedUpdate::Decrease),
            (_, KeyCode::Backspace) => self.turing_machine.reset_memory(),
            (_, KeyCode::Char('p') | KeyCode::Char('P')) => {
                self.turing_machine_running = !self.turing_machine_running
            }
            _ => {}
        }
        Ok(())
    }

    pub fn tick(&mut self) {
        if self.turing_machine_running {
            self.turing_machine.update();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
