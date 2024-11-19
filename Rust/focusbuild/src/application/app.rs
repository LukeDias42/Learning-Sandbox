use std::time::Duration;

use color_eyre::{
    eyre::{Context, OptionExt},
    Result,
};
pub struct App {
    mode: Mode,
    screen_stack: Vec<Screen>,
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
        })
    }
}
