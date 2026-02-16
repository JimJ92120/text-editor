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
use state::{ State, CursorDirection };
use buffer::{ Buffer };

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
            Ok(content) => {
                let mut lines = content.lines()
                    .map(|line| line.trim().to_string())
                    .collect::<Vec<String>>();

                if content.ends_with("\n") {
                    lines.push(String::new());
                }

                lines
            },
            Err(error) => panic!("{}", error),
        };

        Self {
            controller,
            state: State::new(current_file_path_name, content, size().unwrap().into()),
            buffer: Buffer::new(stdout()),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.start()?;

        while self.state.get::<bool>("is_running") {
            self.render()?;
            self.buffer.move_to(self.state.get::<[u16; 2]>("cursor_position"))?;
            self.watch_events()?;
        }

        Ok(())
    }

    fn watch_events(&mut self) -> Result<()> {
        match event::read() {
            Ok(Event::Key(event)) => self.on_keyboard_events(event),
            Ok(Event::Resize(columns, rows)) => self.state.resize([columns, rows]),

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
                KeyCode::Char('s') => {
                    let current_file_path_name = self.state.get::<String>("current_file_path_name");

                    if !current_file_path_name.is_empty() {
                        match self.controller.save_file(
                            current_file_path_name.clone(),
                            self.state.get::<Vec<String>>("content").join("\n")
                        ) {
                            Ok(_) => self.state.prompt(format!("`{}` changes saved.", current_file_path_name)),
                            Err(error) => Err(error),
                        }
                    } else {
                        panic!("no file path found");
                    }
                },
            
                _ => Ok(())
            };
        }

        match event.code {
            KeyCode::Enter => self.state.add_line_break(),
            KeyCode::Backspace => self.state.delete_last_character(),
            KeyCode::Char(character) => self.state.edit(character),

            KeyCode::Up => self.state.move_cursor(CursorDirection::Up),
            KeyCode::Down => self.state.move_cursor(CursorDirection::Down),
            KeyCode::Left => self.state.move_cursor(CursorDirection::Left),
            KeyCode::Right => self.state.move_cursor(CursorDirection::Right),
        
            _ => Ok(())
        }
    }

    fn render(&mut self) -> Result<()> {
        self.buffer.clear()?;

        self.render_footer()?;
        // render content at last to move cursor to content last char
        self.render_content()?;      

        Ok(())
    }

    fn render_content(&mut self) -> Result<()> {
        self.buffer.write_at(
            [0, 0],
            self.state.get::<Vec<String>>("content"),
        )
    }

    fn render_footer(&mut self) -> Result<()> {
        let terminal_size = self.state.get::<[u16; 2]>("terminal_size");

        if 5 >= terminal_size[1] {
            return Ok(());
        }

        let mut content = vec![
            format!("cursor at {:?}", self.state.get::<[u16; 2]>("cursor_position")),
            String::from("[ CTRL + S: save ][ CTRL + Q: quit ]")
        ];
        let footer_offset = 3;
        let prompt = self.state.get::<String>("prompt");

        if !prompt.is_empty() {
            content.push(prompt);

            self.state.clear_prompt()?;
        }

        self.buffer.write_at(
            [0, terminal_size[1] - footer_offset],
            content
        )?;
        

        Ok(())
    }
}
