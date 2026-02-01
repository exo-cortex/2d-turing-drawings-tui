use rand::Rng;

use crate::{memory::Memory2D, ruleset::RuleSet};

pub type State = u8;
pub type Symbol = u8;

#[derive(Debug)]
pub struct TuringMachine<
    const N_STATES: usize,
    const N_SYMBOLS: usize,
    const ROWS: usize,
    const COLS: usize,
> {
    state: State,
    pub rule_set: RuleSet<N_STATES, N_SYMBOLS>,
}

impl<'a, const N_STATES: usize, const N_SYMBOLS: usize, const ROWS: usize, const COLS: usize>
    TuringMachine<N_STATES, N_SYMBOLS, ROWS, COLS>
{
    pub fn new(rng: &mut impl Rng) -> Self {
        TuringMachine {
            state: State::default(),
            rule_set: RuleSet::random(rng),
        }
    }

    pub fn update(&mut self, memory: &mut Memory2D<ROWS, COLS>) {
        let current_symbol = memory.read();
        let (new_state, new_symbol, head_direction) =
            self.rule_set.get_instruction(self.state, current_symbol);
        self.state = *new_state;
        memory.write(*new_symbol);
        memory.move_head(head_direction);
    }

    pub fn randomize_ruleset(&mut self, rng: &mut impl Rng) {
        self.rule_set = RuleSet::random(rng);
    }

    pub fn mutate_ruleset(&mut self, rng: &mut impl Rng) {
        self.rule_set.mutate_ruleset(rng, 3);
    }

    pub fn _print_rules(&self) {
        println!("{}", self.rule_set);
    }
}

// #[derive(Debug)]
// pub struct TuringMachine2<const N_STATES: usize, const N_SYMBOLS: usize> {
//     state: State,
//     pub rule_set: RuleSet<N_STATES, N_SYMBOLS>,
// }

// impl<'a, const N_STATES: usize, const N_SYMBOLS: usize> TuringMachine2<N_STATES, N_SYMBOLS> {
//     pub fn new(rng: &mut impl Rng) -> Self {
//         TuringMachine2 {
//             state: State::default(),
//             rule_set: RuleSet::random(rng),
//         }
//     }

//     pub fn update(&mut self, memory: &mut Memory2D_dynamic) {
//         let current_symbol = memory.read();
//         let (new_state, new_symbol, head_direction) =
//             self.rule_set.get_instruction(self.state, current_symbol);
//         self.state = *new_state;
//         memory.write(*new_symbol);
//         memory.move_head(head_direction);
//     }

//     pub fn randomize_ruleset(&mut self, rng: &mut impl Rng) {
//         self.rule_set = RuleSet::random(rng);
//     }

//     pub fn mutate_ruleset(&mut self, rng: &mut impl Rng) {
//         self.rule_set.mutate_ruleset(rng, 1);
//     }

//     pub fn _print_rules(&self) {
//         println!("{}", self.rule_set);
//     }
// }
