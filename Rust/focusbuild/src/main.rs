mod application;
mod infra;
mod models;

use application::app::App;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal, TerminalOptions, Viewport};
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let backend = CrosstermBackend::new(stdout());
    let terminal_size = Terminal::new(backend)?.size()?;
    let width = terminal_size.width;
    let height = terminal_size.height;

    let viewport = Viewport::Fixed(Rect::new(0, 0, width, height));
    let terminal = ratatui::init_with_options(TerminalOptions { viewport });
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let _ = App::new()?.run(terminal);
    execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");

    ratatui::restore();
    Ok(())
}
