use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Paragraph, Widget},
};

use crate::{app::App, turing_machine::COLUMNS};

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(area);

        Paragraph::new("2D Turing Maching:")
            .fg(Color::Black)
            .bg(Color::Gray)
            .render(layout[0], buf);
        Paragraph::new(
            "Usage: left/right -> double/halve speed, Ctrl+R: randomize rules,\n
            Backspace: Reset memory, M: Mutate rules",
        )
        .bg(Color::DarkGray)
        .render(layout[1], buf);

        let machine_layout =
            Layout::horizontal(vec![Constraint::Min(COLUMNS as u16), Constraint::Min(30)])
                .split(layout[2]);

        self.turing_machine.render(machine_layout[0], buf);
        self.turing_machine
            .tm
            .rule_set
            .render(machine_layout[1], buf);
    }
}
