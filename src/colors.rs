use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn symbol_style<'a>(symbol: u8) -> Span<'a> {
    match symbol {
        0 => Span::raw(" "),
        1 => Span::styled("?", Style::new()),
        2 => Span::styled("O", Style::new().fg(Color::Black).bg(Color::Yellow)),
        3 => Span::styled("#", Style::new().fg(Color::Black).bg(Color::Blue)),
        4 => Span::styled("/", Style::new().fg(Color::Black).bg(Color::LightGreen)),
        5 => Span::styled("*", Style::new().fg(Color::LightRed).bg(Color::Black)),
        6 => Span::styled(";", Style::new().fg(Color::Black).bg(Color::LightRed)),
        7 => Span::styled("%", Style::new().fg(Color::Yellow).bg(Color::DarkGray)),
        8 => Span::styled("-", Style::new().fg(Color::Black).bg(Color::Yellow)),
        9 => Span::styled("$", Style::new().fg(Color::Blue).bg(Color::DarkGray)),
        _ => Span::styled("@", Style::new().fg(Color::Red).bg(Color::Blue)),
    }
}
