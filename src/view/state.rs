use std::{
    io::{ Error, ErrorKind, Result }
};

#[derive(Debug)]
pub struct State {
    is_running: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            is_running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn start(&mut self) -> Result<()> {
        println!("starting...");

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
        println!("stopping...");

        if !self.is_running {
            return Err(Error::new(
                ErrorKind::Other,
                String::from("View is not running.")
            ));
        }

        self.is_running = false;

        Ok(())
    }
}
