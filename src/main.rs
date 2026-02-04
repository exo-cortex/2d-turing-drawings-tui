use crate::app::App;

pub mod app;
mod colors;
pub mod event;
mod memory;
mod ruleset;
mod tm;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
