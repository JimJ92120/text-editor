use std::{
    io::{ Result }
};
use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
        KeyEventKind,
        KeyModifiers,
    }
};

mod state;

use state::{ State };

#[derive(Debug)]
pub struct View {
    state: State,
}

impl View {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.state.start()?;

        while self.state.is_running() {
            self.watch_events()?;
        }

        Ok(())
    }

    fn watch_events(&mut self) -> Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if KeyEventKind::Press != key_event.kind {
                return Ok(());
            }

            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                return match key_event.code {
                    KeyCode::Char('q') => self.state.stop(),
                
                    _ => Ok(())
                };
            } else {
                return match key_event.code {
                    KeyCode::Esc => self.state.stop(),
                
                    _ => Ok(())
                };
            }
        }

        Ok(())
    }
}
