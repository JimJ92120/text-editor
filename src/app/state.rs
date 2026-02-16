use std::{
    io::{ Error, ErrorKind, Result },
    any::{ Any },
};

#[derive(Debug)]
pub struct State {
    is_running: bool,
    content: String,
}

impl State {
    pub fn new(content: String) -> Self {
        Self {
            content,
            is_running: false,
        }
    }

    pub fn get<T: Clone + 'static>(&self, field: &str) -> T {
        let result = match field {
            "content" => Box::new(self.content.clone()) as Box<dyn Any>,
            "is_running" => Box::new(self.is_running.clone()) as Box<dyn Any>,

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
                String::from("View is already running.")
            ));
        }

        self.is_running = true;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.is_running {
            return Err(Error::new(
                ErrorKind::Other,
                String::from("View is not running.")
            ));
        }

        self.is_running = false;

        Ok(())
    }

    pub fn edit(&mut self, character: char) -> Result<()> {
        self.content.push(character);

        Ok(())
    }

    pub fn add_line_break(&mut self) -> Result<()> {
        self.content.push_str("\n");

        Ok(())
    }
}
