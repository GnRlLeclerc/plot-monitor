//! Main application logic

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use pm_lib::Logs;
use ratatui::{DefaultTerminal, prelude::*};

use crate::datasets::draw_datasets;

/// Application state
pub struct App {
    exit: bool,
    logs: Logs,
}

impl App {
    pub fn new(logs: Logs) -> Self {
        App { exit: false, logs }
    }

    /// Main application loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Exit the application
    fn exit(&mut self) {
        self.exit = true;
    }

    /// Rendering function
    fn draw(&mut self, frame: &mut Frame) {
        self.render(frame.area(), frame.buffer_mut());
    }

    /// Crossterm event handling
    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    // Also exit on Ctrl+C
                    KeyCode::Char('c') => {
                        if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                            self.exit()
                        }
                    }
                    // Exit on Esc or 'q'
                    KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            _ => {}
        };

        Ok(())
    }

    /// App rendering function (as a widget)
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        draw_datasets(&self.logs, area, buf);
    }
}
