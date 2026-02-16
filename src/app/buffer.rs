use std::{
    io::{ Result, Write, Stdout },
};

use crossterm::{
    cursor::{ MoveTo },
    terminal::{ Clear, ClearType },
    execute,
};

pub struct BufferLine {
    pub position: [u16; 2],
    pub content: String,
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
        execute!(self.stdout, Clear(ClearType::Purge))?;
        self.move_to([0, 0])?;

        Ok(())
    }

    pub fn resize(&mut self, new_size: [u16; 2]) -> Result<()> {
        self.terminal_size = new_size;

        Ok(())
    }

    pub fn write_to(&mut self, line: BufferLine) -> Result<()> {
        let lines = line.content.lines();
        let lines_count = lines.clone().count();
        
        for (index, content) in lines.clone().enumerate() {
            self.move_to([
                line.position[0],
                line.position[1] + (index as u16)
            ])?;
            write!(self.stdout, "{}",content)?;
        }

        if line.content.ends_with('\n') {
            self.move_to([
                line.position[0],
                (lines_count as u16)
            ])?;
        }

        self.stdout.flush()?;

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
