use std::{
    io::{ Result, Write, Stdout },
};

use crossterm::{
    cursor::{ MoveTo },
    terminal::{ Clear, ClearType },
    execute,
};

pub struct BufferLine<'a> {
    pub position: [u16; 2],
    pub content: &'a str,
}

#[derive(Debug)]
pub struct Buffer {
    stdout: Stdout,
    terminal_size: [u16; 2],
}

impl Buffer {
    pub fn new(stdout: Stdout, terminal_size: [u16; 2]) -> Self {
        Self {
            stdout,
            terminal_size,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.clear()?;

        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        self.move_to(self.terminal_size)?;
        
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))?;

        Ok(())
    }

    pub fn resize(&mut self, new_size: [u16; 2]) -> Result<()> {
        self.terminal_size = new_size;

        Ok(())
    }

    pub fn write_to(&mut self, line: BufferLine) -> Result<()> {
        let lines = line.content.lines();
        for (index, content) in lines.clone().enumerate() {
            self.move_to([
                line.position[0],
                line.position[1] + (index as u16)
            ])?;
            write!(self.stdout, "{}",content)?;
        }

        self.stdout.flush()?;
        self.move_to([
            line.position[0],
            line.position[1] + (lines.count() as u16)
        ])?;

        Ok(())
    }

    fn move_to(&mut self, position: [u16; 2]) -> Result<()> {
        execute!(
            self.stdout,
            MoveTo(position[0], position[1])
        )?;

        Ok(())
    }
}
