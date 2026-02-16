use std::{
    io::{ stdout, Result }
};
use crossterm::{
    terminal::{ size, enable_raw_mode, disable_raw_mode },
    event::{
        self,
        Event,
        KeyEvent,
        KeyCode,
        KeyEventKind,
        KeyModifiers,
    },
};

mod controller;
mod state;
mod buffer;

use controller::{ Controller };
use state::{ State };
use buffer::{ Buffer, BufferLine };

#[derive(Debug)]
pub struct App {
    controller: Controller,
    state: State,
    buffer: Buffer,
}

impl App {
    pub fn new(current_file_path_name: String) -> Self {
        let controller = Controller::new();
        let content = match controller.get_file_content(current_file_path_name.clone()) {
            Ok(content) => content,
            Err(error) => panic!("{}", error),
        };

        Self {
            controller,
            state: State::new(current_file_path_name, content),
            buffer: Buffer::new(stdout(), size().unwrap().into()),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.start()?;

        while self.state.get::<bool>("is_running") {
            self.render()?;
            self.watch_events()?;
        }

        Ok(())
    }

    fn watch_events(&mut self) -> Result<()> {
        match event::read() {
            Ok(Event::Key(event)) => self.on_keyboard_events(event),
            Ok(Event::Resize(columns, rows)) => self.on_resize([columns, rows]),

            _ => Ok(()),
        }
    }

    fn start(&mut self) -> Result<()> {
        self.buffer.start()?;

        enable_raw_mode()?;

        self.state.start()
    }

    fn stop(&mut self) -> Result<()> {
        self.buffer.stop()?;

        disable_raw_mode()?;

        self.state.stop()
    }

    fn on_keyboard_events(&mut self, event: KeyEvent) -> Result<()> {
        if KeyEventKind::Press != event.kind {
            return Ok(());
        }

        if event.modifiers.contains(KeyModifiers::CONTROL) {
            return match event.code {
                KeyCode::Char('q') => self.stop(),
            
                _ => Ok(())
            };
        }

        match event.code {
            KeyCode::Enter => self.state.add_line_break(),
            KeyCode::Backspace => self.state.delete_last_character(),
            KeyCode::Char(character) => self.state.edit(character),
        
            _ => Ok(())
        }
    }

    fn on_resize(&mut self, new_size: [u16; 2]) -> Result<()> {
        self.buffer.resize(new_size)
    }

    fn render(&mut self) -> Result<()> {
        self.buffer.clear()?;

        self.render_commands()?;
        self.render_content()?; // render commands to keep cursor after content

        Ok(())
    }

    fn render_content(&mut self) -> Result<()> {
        self.buffer.write_to(BufferLine {
            position: [0, 0],
            content: self.state.get::<String>("content"),
        })?;

        Ok(())
    }

    fn render_commands(&mut self) -> Result<()> {
        let terminal_size = self.buffer.get::<[u16; 2]>("terminal_size");

        // show if 5 rows minimum
        if 5 <= terminal_size[1] {
            self.buffer.write_to(BufferLine {
                position: [0, terminal_size[1] - 2],
                content: String::from("[ CTRL + S: save ][ CTRL + Q: quit ]")
            })?;
        }

        Ok(())
    }
}
