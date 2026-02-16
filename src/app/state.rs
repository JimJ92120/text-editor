use std::{
    io::{ Error, ErrorKind, Result },
    any::{ Any },
};

#[derive(Debug)]
pub struct State {
    current_file_path_name: String,
    is_running: bool,
    content: Vec<String>,
    prompt: String,
    terminal_size: [u16; 2],
}

impl State {
    pub fn new(current_file_path_name: String, content: Vec<String>, terminal_size: [u16; 2]) -> Self {
        Self {
            current_file_path_name,
            content,
            is_running: false,
            prompt: String::new(),
            terminal_size,
        }
    }

    pub fn get<T: Clone + 'static>(&self, field: &str) -> T {
        let result = match field {
            "content" => Box::new(self.content.clone()) as Box<dyn Any>,
            "is_running" => Box::new(self.is_running.clone()) as Box<dyn Any>,
            "current_file_path_name" => Box::new(self.current_file_path_name.clone()) as Box<dyn Any>,
            "prompt" => Box::new(self.prompt.clone()) as Box<dyn Any>,
            "terminal_size" => Box::new(self.terminal_size.clone()) as Box<dyn Any>,

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
            let last_line_index = self.content.clone().len() - 1;

            self.content[last_line_index].push(character);
        } else {
            self.content.push(character.to_string());
        }

        Ok(())
    }

    pub fn add_line_break(&mut self) -> Result<()> {
        self.content.push(String::new());

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
}
