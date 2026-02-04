use rand::Rng;

use crate::{
    memory::FlexibleMemory2D,
    ruleset::{RuleOutput, RuleSet},
};

const MAX_SPEED: u16 = 1024;

pub type State = u8;
pub type Symbol = u8;

#[derive(Debug)]
pub struct TuringMachine<const N_STATES: usize, const N_SYMBOLS: usize> {
    speed: u16,
    memory: FlexibleMemory2D,
    state: State,
    pub rule_set: RuleSet<N_STATES, N_SYMBOLS>,
}

impl<const N_STATES: usize, const N_SYMBOLS: usize> TuringMachine<N_STATES, N_SYMBOLS> {
    pub fn new(rows: u16, columns: u16, rng: &mut impl Rng) -> Self {
        TuringMachine {
            speed: 1,
            memory: FlexibleMemory2D::from_dimensions(rows, columns),
            state: State::default(),
            rule_set: RuleSet::random(rng),
        }
    }

    pub fn reset_memory(&mut self) {
        self.memory.reset();
    }

    pub fn double_speed(&mut self) {
        if let 1..MAX_SPEED = self.speed {
            self.speed *= 2
        };
    }

    pub fn halve_speed(&mut self) {
        if let 2..=MAX_SPEED = self.speed {
            self.speed /= 2
        }
    }

    pub fn update(&mut self) {
        for _ in 0..self.speed {
            let current_symbol = self.memory.read();
            let RuleOutput(new_state, new_symbol, head_direction) =
                self.rule_set.get_instruction(self.state, current_symbol);
            self.state = *new_state;
            self.memory.write(*new_symbol);
            self.memory.move_head(head_direction);
        }
    }

    pub fn randomize_ruleset(&mut self, rng: &mut impl Rng) {
        self.rule_set = RuleSet::random(rng);
    }

    pub fn mutate_ruleset(&mut self, rng: &mut impl Rng) {
        self.rule_set.mutate_ruleset(rng, 1);
    }

    pub fn resize_memory(&mut self, new_rows: u16, new_columns: u16) {
        self.memory.resize(new_rows, new_columns);
    }

    pub fn get_memory(&self) -> &FlexibleMemory2D {
        &self.memory
    }
}
