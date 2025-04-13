//! Main application logic

use std::{io, thread::spawn};

use crossbeam::{channel::Receiver, select};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use pm_lib::Logs;
use ratatui::{DefaultTerminal, prelude::*};

use crate::datasets::draw_datasets;

/// Application state
pub struct App {
    exit: bool,
    logs: Logs,
    events: Receiver<io::Result<Event>>,
}

impl App {
    pub fn new(logs: Logs) -> Self {
        let (tx, rx) = crossbeam::channel::unbounded::<io::Result<Event>>();

        // Start a background thread to read crossbeam events
        spawn(move || {
            loop {
                tx.send(event::read()).unwrap();
            }
        });

        App {
            exit: false,
            logs,
            events: rx,
        }
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
        // Block until the next event is received
        select! {
            recv(self.logs.updates) -> _ => {},
            recv(self.events) -> event => {
            self.handle_crossterm_event(event.unwrap()?);
            }
        }

        Ok(())
    }

    fn handle_crossterm_event(&mut self, event: Event) {
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
    }

    /// App rendering function (as a widget)
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        draw_datasets(&self.logs, area, buf);
    }
}
