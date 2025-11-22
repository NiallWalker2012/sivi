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
use crate::core::input::draw;

use std::io::{
    Result,
    Write,
    stdout
};

pub struct FileConts {
    pub buffer: Vec<String>,
    pub x_pos: u32,
    pub y_pos: u32,
    pub f_name: String,     //File name
    line: u32,  
}

impl FileConts {
    fn new(file_name: String) -> Self {
        Self {
            buffer: vec![String::new()],
            x_pos: 0,
            y_pos: 0,
            f_name: file_name,
            line: 1,
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
}

pub fn get_input(file_contents: String, f_name: String) -> Result<()> {
    let mut contents = FileConts::new(f_name);
    
    if let Err(why) = contents.vectorize(file_contents) {
        eprintln!("Failed to vectorize contents: {why}");
        return Ok(());
    }
    

    disable_raw_mode()?;
    for item in contents.buffer.clone() {
        println!("{}. {}", contents.line, item);      //In the mean time, this is just here for visualisation and testing
        contents.line += 1;
    }
    enable_raw_mode()?;

    /*if let Err(why) = draw::draw(contents) {
        eprintln!("Error whilst drawing contents: {}", why);
        return Ok(());
    }
    */

    Ok(())
}
