use std::{
    io::{ Result, Write, Stdout },
};

use crossterm::{
    cursor::{ MoveTo },
    terminal::{ Clear, ClearType },
    execute,
};

#[derive(Debug)]
pub struct Buffer {
    stdout: Stdout,
}

impl Buffer {
    pub fn new(stdout: Stdout) -> Self {
        Self {
            stdout,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.clear()?;

        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        self.clear()?;
        
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))?;
        execute!(self.stdout, Clear(ClearType::Purge))?;
        self.move_to([0, 0])?;

        Ok(())
    }

    pub fn write_at(&mut self, position: [u16; 2], content: Vec<String>) -> Result<()> {
        self.move_to(position)?;
        
        for (line_index, line_content) in content.iter().enumerate() {
            self.move_to([0, position[1] + (line_index as u16)])?;
            write!(self.stdout, "{}", line_content)?;
        }

        self.stdout.flush()?;

        Ok(())
    }

    pub fn move_to(&mut self, position: [u16; 2]) -> Result<()> {
        execute!(
            self.stdout,
            MoveTo(position[0], position[1])
        )?;

        Ok(())
    }
}
