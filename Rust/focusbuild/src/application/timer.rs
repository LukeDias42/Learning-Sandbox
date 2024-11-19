use chrono::{DateTime, Local};
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use std::{fmt::Debug, time::Instant};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
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
                self.end_focus_session();
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
    pub fn formatted_timer(mut elapsed_seconds: i64) -> String {
        let is_negative = elapsed_seconds < 0;
        if is_negative {
            elapsed_seconds *= -1;
        }
        let hours = elapsed_seconds / 3600;
        let minutes = (elapsed_seconds / 60) - hours * 60;
        let seconds = (elapsed_seconds) - minutes * 60 - hours * 3600;
        let negative_string = if is_negative { "-" } else { "" };
        format!("{negative_string}{hours:02}:{minutes:02}:{seconds:02}")
    }

    fn end_focus_session(&self) -> Result<()> {
        let time_focusing = self.focus_time.iter().sum();
        let time_chilling = self.break_time.iter().sum();
        self.focus_session_repository.insert_focus_session(
            self.start_date,
            time_focusing,
            time_chilling,
        )?;
        Ok(())
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

        let timers_area = Rect::new(
            3 * area.width / 8,
            7 * area.height / 16,
            area.width / 4,
            area.height / 8,
        );
        let timers_block = Block::new()
            .borders(Borders::ALL)
            .padding(Padding::symmetric(2, 3));
        timers_block.render(timers_area, buf);

        let focus_timer_area = Rect::new(
            timers_area.x + 1,
            timers_area.y + 1,
            timers_area.width / 3 - 1,
            timers_area.height - 2,
        );
        let previous_focus_time = self
            .focus_time
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();
        let focus_seconds = previous_focus_time
            + if self.is_focusing {
                self.stopwatch.elapsed().as_secs()
            } else {
                0
            };
        let focus_timer_block = Paragraph::new(Timer::formatted_timer(focus_seconds as i64))
            .block(Block::new().borders(Borders::ALL).title("Focus"))
            .style(if self.is_focusing {
                THEME.timer.active
            } else {
                THEME.timer.inactive
            });
        focus_timer_block.render(focus_timer_area, buf);

        let break_timer_area = Rect::new(
            focus_timer_area.x + focus_timer_area.width + 1,
            focus_timer_area.y,
            focus_timer_area.width,
            focus_timer_area.height,
        );
        let previous_break_time = self
            .break_time
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();

        let break_seconds = previous_break_time
            + if self.is_focusing {
                0
            } else {
                self.stopwatch.elapsed().as_secs()
            };
        let break_timer_block = Paragraph::new(Timer::formatted_timer(break_seconds as i64))
            .block(Block::new().borders(Borders::ALL).title("Break"))
            .style(if self.is_focusing {
                THEME.timer.inactive
            } else {
                THEME.timer.active
            });
        break_timer_block.render(break_timer_area, buf);

        let balance_timer_area = Rect::new(
            break_timer_area.x + break_timer_area.width + 1,
            break_timer_area.y,
            break_timer_area.width,
            break_timer_area.height,
        );
        let balance_seconds = (focus_seconds as i64 / 3) - break_seconds as i64;

        let balance_timer_block = Paragraph::new(Timer::formatted_timer(balance_seconds))
            .block(Block::new().borders(Borders::ALL).title("Break"))
            .style(THEME.timer.active);
        balance_timer_block.render(balance_timer_area, buf);
    }
}
