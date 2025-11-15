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
        Stylize 
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
    Write,
};

struct FileConts {
    buffer: Vec<String>,
    x_pos: u32,
    y_pos: u32,
    f_name: String,     //File name
    //curr_line: usize,  (until line numbers are impltemented in load.rs, this feature will not be
    //used)
}

impl FileConts {
    fn new() -> Self {
        Self {
            buffer: vec![String::new()],
            x_pos: 0,
            y_pos: 0,
        }
    }

    fn vectorize(&mut self, contents: String) -> Result<()> {
        //Split string contents into a vector (by lines)
        self.buffer = contents.lines().map(|conts| conts.to_string()).collect();
        if self.buffer.is_empty() {
            self.buffer.push(String::new());
        }
        
        Ok(())
    }

    fn draw(&self) -> crossterm::Result<()> {
        let (width, height) = terminal::size()?;
        let height = height as u16;
        //Leave last line for cleaner TUI 
        let text_height = if height > 1 { height - 1 } else { 0 };

        execute!(stdout(), terminal::Clear(ClearType::All))?;

        for (i, line) in self.buffer.iter().take(text_height).enumerate() {
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
    }
}

pub fn get_input(file_contents: String) -> Result<()> {
    let mut contents = FileConts::new();
    
    contents.vectorize(file_contents);

    for i in contents.buffer {
        println!("{}", i);
    }
    Ok(())
}
