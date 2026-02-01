use {
    crate::{colors::symbol_style, memory::Memory2D, tm::TuringMachine},
    rand::{SeedableRng, rngs::SmallRng},
    ratatui::{
        layout::{Constraint, Layout},
        text::{Line, Span},
        widgets::{Paragraph, Widget},
    },
};

const STATES: usize = 2;
pub const SYMBOLS: usize = 8;

pub const ROWS: usize = 32;
pub const COLUMNS: usize = 128;

const MAX_SPEED: u16 = 1024;

pub enum SpeedUpdate {
    Increase,
    Decrease,
}

#[derive(Debug)]
pub struct TuringMachineInstance {
    speed: u16,
    rng: SmallRng,
    memory: Memory2D<ROWS, COLUMNS>,
    pub tm: TuringMachine<STATES, SYMBOLS, ROWS, COLUMNS>,
    // pub tm: TuringMachine<STATES, SYMBOLS>,
}

impl Default for TuringMachineInstance {
    fn default() -> Self {
        let mut rng = SmallRng::seed_from_u64(0);
        TuringMachineInstance {
            speed: 1,
            rng: SmallRng::seed_from_u64(0),
            memory: Memory2D::new(),
            tm: TuringMachine::new(&mut rng),
        }
    }
}

impl TuringMachineInstance {
    pub fn new(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        TuringMachineInstance {
            speed: 1,
            rng: SmallRng::seed_from_u64(seed),
            memory: Memory2D::new(),
            tm: TuringMachine::new(&mut rng),
        }
    }

    pub fn change_speed(&mut self, change_speed: SpeedUpdate) {
        match (self.speed, change_speed) {
            (1..MAX_SPEED, SpeedUpdate::Increase) => self.speed *= 2,
            (2..=MAX_SPEED, SpeedUpdate::Decrease) => self.speed /= 2,
            _ => {}
        }
    }

    pub fn update(&mut self) {
        (0..self.speed).for_each(|_| {
            self.tm.update(&mut self.memory);
        })
    }

    pub fn randomize_ruleset(&mut self) {
        self.tm.randomize_ruleset(&mut self.rng);
    }

    pub fn mutate_ruleset(&mut self) {
        self.tm.mutate_ruleset(&mut self.rng);
    }

    pub fn reset_memory(&mut self) {
        self.memory.reset();
    }
}

impl Widget for &TuringMachineInstance {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical(
            (0..ROWS)
                .map(|_| Constraint::Length(1))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);

        layout
            .iter()
            .zip(self.memory._get().iter())
            .for_each(|(&row_layout, row)| {
                let text_row = row
                    .iter()
                    .map(|el| symbol_style(*el))
                    .collect::<Vec<Span>>();

                Paragraph::new(Line::from(text_row)).render(row_layout, buf);
            });
    }
}
