use std::{
    io::{ Error, ErrorKind, Result },
    any::{ Any },
};

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct State {
    current_file_path_name: String,
    is_running: bool,
    content: Vec<String>,
    prompt: String,
    terminal_size: [u16; 2],
    cursor_position: [u16; 2],
}

impl State {
    pub fn new(current_file_path_name: String, content: Vec<String>, terminal_size: [u16; 2]) -> Self {
        let last_line_index = content.len();
        let cursor_position: [u16; 2] = if 0 < last_line_index {
            [
                content[last_line_index - 1].len() as u16,
                (last_line_index as u16) - 1
            ]
        } else {
            [0, 0]
        };

        Self {
            current_file_path_name,
            content,
            is_running: false,
            prompt: String::new(),
            terminal_size,
            cursor_position,
        }
    }

    pub fn get<T: Clone + 'static>(&self, field: &str) -> T {
        let result = match field {
            "content" => Box::new(self.content.clone()) as Box<dyn Any>,
            "is_running" => Box::new(self.is_running.clone()) as Box<dyn Any>,
            "current_file_path_name" => Box::new(self.current_file_path_name.clone()) as Box<dyn Any>,
            "prompt" => Box::new(self.prompt.clone()) as Box<dyn Any>,
            "terminal_size" => Box::new(self.terminal_size.clone()) as Box<dyn Any>,
            "cursor_position" => Box::new(self.cursor_position.clone()) as Box<dyn Any>,

            _ => panic!("`{}` field doesn't exist.", field),
        };

        result
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }

    pub fn start(&mut self) -> Result<()> {
        if self.is_running {
            return Err(Error::new(
                ErrorKind::Other,
                String::from("View is already running.\n")
            ));
        }

        self.is_running = true;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.is_running {
            return Err(Error::new(
                ErrorKind::Other,
                String::from("View is not running.\n")
            ));
        }

        self.is_running = false;

        Ok(())
    }

    pub fn edit(&mut self, character: char) -> Result<()> {
        if !self.content.is_empty() {
            let current_line = self.content[self.cursor_position[1] as usize].clone();

            if 0 == current_line.len()
                || self.cursor_position[0] as usize == current_line.len() - 1
            {
                self.content[self.cursor_position[1] as usize].push(character);
            } else {
                let (before, after) = current_line.split_at(self.cursor_position[0] as usize);

                self.content[self.cursor_position[1] as usize] = format!("{}{}{}", before, character, after);
            }
        } else {
            self.content.push(character.to_string());
        }

        self.cursor_position[0] += 1;

        Ok(())
    }

    pub fn add_line_break(&mut self) -> Result<()> {
        let current_line = self.content[self.cursor_position[1] as usize].clone();

        if 0 == current_line.len() {
            self.content.insert(self.cursor_position[1] as usize, String::new());
            self.cursor_position[0] = 0;
        } else {
            let (before, after) = current_line.split_at(self.cursor_position[0] as usize);

            self.content[self.cursor_position[1] as usize] = before.to_string();
            self.content.insert(self.cursor_position[1] as usize + 1, after.to_string());
            self.cursor_position[0] = after.len() as u16;
        }

        self.cursor_position[1] += 1;

        Ok(())
    }

    pub fn delete_last_character(&mut self) -> Result<()> {
        if !self.content.is_empty() {
            self.content.pop();
        }

        Ok(())
    }

    pub fn prompt(&mut self, content: String) -> Result<()> {
        self.prompt = content;

        Ok(())
    }

    pub fn clear_prompt(&mut self) -> Result<()> {
        self.prompt = String::new();

        Ok(())
    }

    pub fn resize(&mut self, new_size: [u16; 2]) -> Result<()> {
        self.terminal_size = new_size;

        Ok(())
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) -> Result<()> {
        if self.content.is_empty() {
            return Ok(());
        }

        let current_position = self.cursor_position;
        let content_length = self.content.len() as u16;
        let mut new_position = current_position.clone();

        match direction {
            CursorDirection::Up => {
                if 0 < current_position[1] {
                    new_position[1] = current_position[1] - 1;

                    let current_line = &self.content[new_position[1] as usize];

                    new_position[0] = if !current_line.is_empty() {
                        current_line.len() as u16
                    } else {
                        0
                    }
                }
            },
            CursorDirection::Down => {
                if content_length - 1 > current_position[1] {
                    new_position[1] = current_position[1] + 1;

                    let current_line = &self.content[new_position[1] as usize];

                    new_position[0] = if !current_line.is_empty() {
                        current_line.len() as u16
                    } else {
                        0
                    }
                }
            },
            CursorDirection::Left => {
                if 0 < current_position[0] {
                    new_position[0] -= 1;
                }
            },
            CursorDirection::Right => {
                let current_line = &self.content[current_position[1] as usize];

                if !current_line.is_empty()
                    && current_line.len() - 1 > current_position[0] as usize
                {
                    new_position[0] += 1;
                }
            }
        };

        if current_position != new_position {
            self.cursor_position = new_position;
        }

        Ok(())
    }
}
