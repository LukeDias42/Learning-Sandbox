use chrono::{DateTime, Local};
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use std::time::Instant;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Widget},
};

use crate::{
    application::theme::THEME,
    infra::repositories::focus_session_repository::FocusSessionRepository,
};

use super::app::{KeyPressResult, Mode, RemoveFromStack, Screen};

const FOCUS_TITLE_HEIGHT: u16 = 6;
const FOCUS_TITLE_WIDTH: u16 = 45;
const FOCUS_TITLE: &str = "███████╗ ██████╗  ██████╗██╗   ██╗███████╗██╗
██╔════╝██╔═══██╗██╔════╝██║   ██║██╔════╝██║
█████╗  ██║   ██║██║     ██║   ██║███████╗██║
██╔══╝  ██║   ██║██║     ██║   ██║╚════██║╚═╝
██║     ╚██████╔╝╚██████╗╚██████╔╝███████║██╗
╚═╝      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝╚═╝";
const DIGIT_SIZE_MAP: [(&str, u16); 12] = [
    (
        " ████ 
██  ██
██  ██
██  ██
 ████ ",
        6,
    ),
    (
        "████  
  ██  
  ██  
  ██  
██████",
        6,
    ),
    (
        " ████ 
██  ██
   ██ 
 ██   
██████",
        6,
    ),
    (
        " ████ 
██  ██
   ██
██  ██
 ████ ",
        6,
    ),
    (
        "██  ██
██  ██
██████
    ██
    ██",
        6,
    ),
    (
        "██████
██    
█████ 
    ██
█████ ",
        6,
    ),
    (
        " ████ 
██    
█████ 
██  ██
 ████ ",
        6,
    ),
    (
        "██████
   ██ 
  ██  
 ██   
██    ",
        6,
    ),
    (
        " ████ 
██  ██
 ████ 
██  ██
 ████ ",
        6,
    ),
    (
        " ████ 
██  ██
 █████
    ██
 ████ ",
        6,
    ),
    (
        "  
██
  
██
    ",
        2,
    ),
    (
        "   
   
███
   
      ",
        3,
    ),
];
pub struct Timer {
    stopwatch: Instant,
    focus_time: Vec<u64>,
    break_time: Vec<u64>,
    start_date: DateTime<Local>,
    is_focusing: bool,
    focus_session_repository: FocusSessionRepository,
}

impl Timer {
    pub fn new() -> Result<Self> {
        let timer = Self {
            stopwatch: Instant::now(),
            focus_time: Vec::new(),
            break_time: Vec::new(),
            start_date: Local::now(),
            is_focusing: true,
            focus_session_repository: FocusSessionRepository::new()?,
        };
        Ok(timer)
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.end_focus_session()?;
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Char('b') => {
                if self.is_focusing {
                    self.focus_time.push(self.stopwatch.elapsed().as_secs());
                    self.is_focusing = false;
                    self.stopwatch = Instant::now();
                }
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('f') => {
                if !self.is_focusing {
                    self.break_time.push(self.stopwatch.elapsed().as_secs());
                    self.is_focusing = true;
                    self.stopwatch = Instant::now();
                }
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false)),
        })
    }

    fn get_number_digits(mut number: i64) -> Vec<usize> {
        let mut digits = Vec::new();
        while number != 0 {
            digits.push((number % 10) as usize);
            number /= 10;
        }
        digits.reverse();
        digits
    }
    pub fn timer_digits_size_vector(mut elapsed_seconds: i64) -> Vec<(String, u16)> {
        let mut timer_digits = Vec::new();
        if elapsed_seconds < 0 {
            timer_digits.push((DIGIT_SIZE_MAP[11].0.to_string(), DIGIT_SIZE_MAP[11].1));
            elapsed_seconds *= -1;
        }

        let hours = elapsed_seconds / 3600;
        if hours < 10 {
            timer_digits.push((DIGIT_SIZE_MAP[0].0.to_string(), DIGIT_SIZE_MAP[0].1));
            if hours == 0 {
                timer_digits.push((DIGIT_SIZE_MAP[0].0.to_string(), DIGIT_SIZE_MAP[0].1));
            }
        }
        for number in Timer::get_number_digits(hours) {
            timer_digits.push((
                DIGIT_SIZE_MAP[number].0.to_string(),
                DIGIT_SIZE_MAP[number].1,
            ));
        }
        timer_digits.push((DIGIT_SIZE_MAP[10].0.to_string(), DIGIT_SIZE_MAP[10].1));

        let minutes = (elapsed_seconds / 60) - hours * 60;

        if minutes < 10 {
            timer_digits.push((DIGIT_SIZE_MAP[0].0.to_string(), DIGIT_SIZE_MAP[0].1));
            if minutes == 0 {
                timer_digits.push((DIGIT_SIZE_MAP[0].0.to_string(), DIGIT_SIZE_MAP[0].1));
            }
        }
        for number in Timer::get_number_digits(minutes) {
            timer_digits.push((
                DIGIT_SIZE_MAP[number].0.to_string(),
                DIGIT_SIZE_MAP[number].1,
            ));
        }
        return timer_digits;
    }

    fn get_focus_time(&self) -> u64 {
        let previous_focus_time = self
            .focus_time
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();
        previous_focus_time
            + if self.is_focusing {
                self.stopwatch.elapsed().as_secs()
            } else {
                0
            }
    }
    fn get_break_time(&self) -> u64 {
        let previous_break_time = self
            .break_time
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();
        previous_break_time
            + if self.is_focusing {
                0
            } else {
                self.stopwatch.elapsed().as_secs()
            }
    }

    fn end_focus_session(&self) -> Result<()> {
        let focus_seconds = self.get_focus_time();
        if focus_seconds < 60 {
            return Ok(());
        }

        self.focus_session_repository.insert_focus_session(
            self.start_date,
            focus_seconds,
            self.get_break_time(),
        )?;
        Ok(())
    }

    pub fn draw_timers(&self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .borders(Borders::ALL)
            .padding(Padding::symmetric(2, 3))
            .style(THEME.timer.title)
            .render(area, buf);

        let focus_block_area = Rect::new(area.x + 1, area.y + 1, 34, 7);
        let focus_seconds = self.get_focus_time();
        Block::new()
            .borders(Borders::ALL)
            .title("focus")
            .style(if self.is_focusing {
                THEME.timer.active
            } else {
                THEME.timer.inactive
            })
            .render(focus_block_area, buf);
        let focus_digits = Timer::timer_digits_size_vector(focus_seconds as i64);
        let mut focus_digit_area = Rect::new(focus_block_area.x + 2, focus_block_area.y + 1, 6, 5);
        for digit_size in focus_digits {
            Text::raw(digit_size.0)
                .style(if self.is_focusing {
                    THEME.timer.active
                } else {
                    THEME.timer.inactive
                })
                .render(focus_digit_area, buf);
            focus_digit_area.x += digit_size.1 + 1;
        }

        let break_block_area = Rect::new(
            focus_block_area.x + focus_block_area.width + 1,
            focus_block_area.y,
            focus_block_area.width,
            focus_block_area.height,
        );
        let break_seconds = self.get_break_time();
        Block::new()
            .borders(Borders::ALL)
            .title("break")
            .style(if self.is_focusing {
                THEME.timer.inactive
            } else {
                THEME.timer.active
            })
            .render(break_block_area, buf);
        let break_digits = Timer::timer_digits_size_vector(break_seconds as i64);
        let mut break_digits_area = Rect::new(break_block_area.x + 2, break_block_area.y + 1, 6, 5);
        for digit_size in break_digits {
            Text::raw(digit_size.0)
                .style(if self.is_focusing {
                    THEME.timer.inactive
                } else {
                    THEME.timer.active
                })
                .render(break_digits_area, buf);
            break_digits_area.x += digit_size.1 + 1;
        }

        let balance_block_area = Rect::new(
            break_block_area.x + break_block_area.width + 1,
            break_block_area.y,
            37,
            7,
        );
        let balance_seconds = self.get_focus_time() as i64 / 3 - self.get_break_time() as i64;
        Block::new()
            .borders(Borders::ALL)
            .title("balance")
            .style(THEME.timer.inactive)
            .render(balance_block_area, buf);
        let balance_digits = Timer::timer_digits_size_vector(balance_seconds as i64);
        let mut balance_digit_area =
            Rect::new(balance_block_area.x + 1, balance_block_area.y + 1, 6, 5);
        for digit_size in balance_digits {
            Text::raw(digit_size.0)
                .style(THEME.timer.inactive)
                .render(balance_digit_area, buf);
            balance_digit_area.x += digit_size.1 + 1;
        }
    }

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_area = Rect::new(
            area.width / 4 + (area.width / 4 - FOCUS_TITLE_WIDTH / 2),
            area.height / 4 - FOCUS_TITLE_HEIGHT,
            area.width / 2,
            area.height / 2,
        );
        Text::raw(FOCUS_TITLE)
            .style(THEME.timer.title)
            .render(title_area, buf);

        let timers_area = Rect::new((area.width - 109) / 2, title_area.y + 6, 109, 9);
        self.draw_timers(timers_area, buf);
    }
}
