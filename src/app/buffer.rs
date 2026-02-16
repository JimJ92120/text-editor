use std::{
    io::{ Result, Write, Stdout },
    any::{ Any }
};

use crossterm::{
    cursor::{ MoveTo },
    terminal::{ Clear, ClearType },
    execute,
};

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

    pub fn get<T: Clone + 'static>(&self, field: &str) -> T {
        let result = match field {
            "terminal_size" => Box::new(self.terminal_size.clone()) as Box<dyn Any>,

            _ => panic!("`{}` field doesn't exist.", field),
        };

        result
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }

    pub fn start(&mut self) -> Result<()> {
        self.clear()?;

        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        self.clear()?;
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

    pub fn write_at(&mut self, position: [u16; 2], content: String) -> Result<()> {
        let lines = content.lines();
        let lines_count = lines.clone().count();
        
        if 0 < lines_count {
            for (line_index, line_content) in lines.clone().enumerate() {
                self.move_to([
                    position[0],
                    position[1] + (line_index as u16)
                ])?;
                write!(self.stdout, "{}", line_content)?;
            }
        } else {
            self.move_to(position)?;
        }

        if content.ends_with('\n') {
            self.move_to([
                position[0],
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
