use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{
    colors::symbol_style,
    tm::{State, Symbol},
};

use {rand::Rng, std::fmt::Display};

const N_DIRECTIONS: u8 = 4;

#[derive(Debug, Default, Copy, Clone)]
pub enum HeadDirection {
    #[default]
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl HeadDirection {
    fn from_byte(value: u8) -> Self {
        match value {
            0 => HeadDirection::Top,
            1 => HeadDirection::Right,
            2 => HeadDirection::Bottom,
            3 => HeadDirection::Left,
            _ => panic!(),
        }
    }
}

impl Display for HeadDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadDirection::Right => {
                write!(f, "right")
            }
            HeadDirection::Bottom => {
                write!(f, " down")
            }
            HeadDirection::Left => {
                write!(f, " left")
            }
            HeadDirection::Top => {
                write!(f, "   up")
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct RuleSet<const N_STATES: usize, const N_SYMBOLS: usize> {
    pub rules: Vec<((State, Symbol), (State, Symbol, HeadDirection))>,
}

impl<const N_STATES: usize, const N_SYMBOLS: usize> RuleSet<N_STATES, N_SYMBOLS> {
    pub fn new() -> Self {
        let ruleset = (0..N_STATES as u8)
            .flat_map(move |st| {
                (0..N_SYMBOLS as u8).map(move |sy| {
                    (
                        (st, sy),
                        (
                            State::default(),
                            Symbol::default(),
                            HeadDirection::default(),
                        ),
                    )
                })
            })
            .collect::<Vec<((u8, u8), (u8, u8, HeadDirection))>>();
        RuleSet { rules: ruleset }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let ruleset = (0..N_STATES as u8)
            .flat_map(move |st| (0..N_SYMBOLS as u8).map(move |sy| (st, sy)))
            .into_iter()
            .map(|(st, sy)| {
                (
                    (st, sy),
                    (
                        rng.random_range(0..N_STATES as u8),
                        rng.random_range(0..N_SYMBOLS as u8),
                        HeadDirection::from_byte(rng.random_range(0..4)),
                    ),
                )
            })
            .collect::<Vec<((u8, u8), (u8, u8, HeadDirection))>>();
        RuleSet { rules: ruleset }
    }

    pub fn mutate_ruleset(&mut self, rng: &mut impl Rng, times: usize) {
        for _ in 0..times {
            let random_rule = &mut self.rules[rng.random_range(0..N_STATES * N_SYMBOLS)];
            match rng.random_range(0..3) {
                0 => {
                    random_rule.1.0 = (random_rule.1.0 + 1).rem_euclid(N_STATES as u8);
                }
                1 => {
                    random_rule.1.1 = (random_rule.1.1 + 1).rem_euclid(N_SYMBOLS as u8);
                }
                2 => {
                    random_rule.1.2 = match random_rule.1.2 {
                        HeadDirection::Top => HeadDirection::Right,
                        HeadDirection::Right => HeadDirection::Bottom,
                        HeadDirection::Bottom => HeadDirection::Left,
                        HeadDirection::Left => HeadDirection::Top,
                    }
                }
                _ => {}
            }
        }
    }

    pub fn _get_instructions(
        &self,
        in_state: &State,
        in_symbol: &Symbol,
    ) -> &(State, Symbol, HeadDirection) {
        &self
            .rules
            .iter()
            .find(|rule| rule.0 == (*in_state, *in_symbol))
            .unwrap()
            .1
    }

    pub fn get_instruction(
        &self,
        in_state: State,
        in_symbol: Symbol,
    ) -> &(State, Symbol, HeadDirection) {
        &self.rules[(in_state as usize) * N_SYMBOLS + in_symbol as usize].1
    }
}

impl<const N_STATES: usize, const N_SYMBOLS: usize> Display for RuleSet<N_STATES, N_SYMBOLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ruleset for Turing machine with {} states and {} symbols",
            N_STATES, N_SYMBOLS,
        )
        .unwrap();
        self.rules.iter().for_each(|&rule| {
            writeln!(
                f,
                "[{}, {}] -> [{}, {}, {}]",
                rule.0.0, rule.0.1, rule.1.0, rule.1.1, rule.1.2
            )
            .unwrap();
        });
        Ok(())
    }
}

impl<const N_STATES: usize, const N_SYMBOLS: usize> Widget for &RuleSet<N_STATES, N_SYMBOLS> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical(vec![Constraint::Length(1), Constraint::Min(1)]).split(area);

        Paragraph::new(format!("{} Rules in set of rules:", self.rules.len()))
            .fg(Color::Black)
            .bg(Color::LightRed)
            .render(layout[0], buf);

        // layout
        //     .iter()
        //     .zip(self.rules.iter())
        //     .enumerate()
        //     .for_each(|(idx, (l, r))| {
        //         rule_display(idx, *r).left_aligned().render(*l, buf);
        //     });

        let list_of_rules = self
            .rules
            .iter()
            .enumerate()
            .map(|(idx, r)| rule_display(idx, *r).left_aligned())
            .collect::<Vec<Line>>();

        Paragraph::new(list_of_rules).render(layout[1], buf);
        // .render(*l, buf);
        // });
    }
}

fn rule_display<'a>(idx: usize, rule: ((u8, u8), (u8, u8, HeadDirection))) -> Line<'a> {
    let input_state = Span::raw(format!("{}", rule.0.0));
    let input_symbol = symbol_style(rule.0.1);

    let output_state = Span::raw(format!("{}", rule.1.0));
    let output_symbol = symbol_style(rule.1.1);
    let output_direction = Span::raw(format!("{}", rule.1.2));

    Line::from(vec![
        format!("{:02}: ", idx).into(),
        "(".into(),
        input_state,
        ",".into(),
        input_symbol,
        ") -> (".into(),
        output_state,
        ",".into(),
        output_symbol,
        ",".into(),
        output_direction,
        ")".into(),
    ])
}
