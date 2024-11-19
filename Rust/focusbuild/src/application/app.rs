use std::time::Duration;

use color_eyre::{
    eyre::{Context, OptionExt},
    Result,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
    DefaultTerminal, Frame,
};

use crate::application::{
    main_menu::MainMenu, statistics::Statistics, theme::THEME, timer::Timer, town::Town,
};

pub struct App {
    mode: Mode,
    screen_stack: Vec<Screen>,
    pub main_menu: MainMenu,
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    MainMenu,
    Timer,
    Town,
    Statistics,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Running,
    Destroy,
    Quit,
}

impl App {
    pub fn new() -> Result<App> {
        let mut screen_stack: Vec<Screen> = Vec::new();
        screen_stack.push(Screen::default());
        Ok(App {
            mode: Mode::default(),
            screen_stack,
            main_menu: MainMenu::default(),
        })
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
        let current_screen = self.screen_stack.last().unwrap_or(&Screen::None).to_owned();
        if current_screen == Screen::None {
            self.mode = Mode::Quit;
            return Ok(());
        }
        let key_press_result = KeyPressResult(current_screen, self.mode, RemoveFromStack(false));
        let result = match current_screen {
            Screen::MainMenu => self.main_menu.handle_key_press(key)?,
            _ => key_press_result,
        };
        self.mode = result.1;
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn is_running(&self) -> bool {
        self.mode == Mode::Running
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().style(THEME.root);
        block.render(area, buf);
        if let Some(screen) = self.screen_stack.last() {
            match screen {
                Screen::MainMenu => self.main_menu.render(area, buf),
                Screen::Timer => self.timer.render(area, buf),
                Screen::Statistics => self.statistics.render(area, buf),
                Screen::Town => self.town.render(area, buf),
                _ => {}
            }
        }
    }
}
