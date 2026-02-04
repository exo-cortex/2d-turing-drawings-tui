use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Paragraph, Widget},
};

use crate::app::{App, RULE_PANE_WIDTH};

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical(vec![
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(area);

        Paragraph::new(
            "2d Turing Machine - Usage:\nleft/right -> decrease/increase speed\nBackspace: Reset memory\nM: Mutate rules, R: randomize rules",
        )
        .bg(Color::DarkGray)
        .render(layout[0], buf);

        let machine_layout = Layout::horizontal(vec![
            Constraint::Min(15),
            Constraint::Length(RULE_PANE_WIDTH),
        ])
        .split(layout[1]);

        self.tm.get_memory().render(machine_layout[0], buf);
        self.tm.rule_set.render(machine_layout[1], buf);
    }
}
