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
};
use std::io::Result;

struct Contents {
    buffer: Vec<String>,
    x_pos: u32,
    y_pos: u32,
    //curr_line: usize,  (until line numbers are impltemented in load.rs, this feature will not be
    //used)
}



pub fn get_input(contents: String) -> Result<()> {
    
    Ok(())
}
