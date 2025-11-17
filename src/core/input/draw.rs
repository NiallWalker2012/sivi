use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode 
    },
    execute,
    style::{
        Color,
        Stylize,
        Print,
    },
    terminal::{
        self,
        ClearType,
        disable_raw_mode,
        enable_raw_mode
    },
    queue,
};

use std::io::{
    Result,
    stdout,
};

use crate::core::input::input::FileConts;

pub fn draw(contents: FileConts) -> Result<()> {
        let (width, height) = terminal::size()?;
        let height = height as usize;
        //Leave last line for cleaner TUI 
        let text_height = if height > 1 { height - 1 } else { 0 };

        execute!(stdout(), terminal::Clear(ClearType::All))?;

        for (i, line) in contents.buffer.clone().iter().take(text_height).enumerate() {
            queue!(
                stdout(),
                cursor::MoveTo(0, i as u16),
                Print(if line.len() > width as usize {
                    //Trime appropriately
                    let mut pos = line.clone();
                    pos.truncate(width as usize);
                    pos
                } else {
                    line.clone()
                })
            )?;
        }
        Ok(())
    }

