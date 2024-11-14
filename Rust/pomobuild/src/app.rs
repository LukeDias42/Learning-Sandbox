use std::{default, time::Duration};

use color_eyre::{eyre::Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
    DefaultTerminal, Frame,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct App {
    mode: Mode,
    width: u16,
    height: u16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Running,
    Destroy,
    Quit,
}

const CHARCOAL_BROWN: Color = Color::Rgb(60, 48, 42);
const MOSS_GREEN: Color = Color::Rgb(138, 154, 91);
impl App {
    pub fn new(width: u16, height: u16) -> App {
        App {
            mode: Mode::Running,
            width,
            height,
        }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal
                .draw(|frame| self.draw(frame))
                .wrap_err("terminal.draw")?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return self.handle_key_press(key)
            }
            _ => {}
        }
        return Ok(());
    }

    fn handle_key_press(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.mode = Mode::Quit,
            _ => {}
        };
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::new().style(Style::new().bg(CHARCOAL_BROWN));
        let rect = Rect::new(0, 0, self.width, self.height);
        frame.render_widget(block, rect);
        let block = Block::new().style(Style::new().bg(MOSS_GREEN));
        let rect = Rect::new(
            self.width / 4,
            self.height / 4,
            self.width / 2,
            self.height / 2,
        );
        frame.render_widget(block, rect);
    }

    fn is_running(&self) -> bool {
        self.mode == Mode::Running
    }
}
