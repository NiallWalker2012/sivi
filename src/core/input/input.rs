use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode,
        KeyModifiers,
        KeyEvent,
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
use std::time::Duration;

pub struct FileConts {
    pub buffer: Vec<String>,
    pub x_pos: u32,
    pub y_pos: u32,
    pub f_name: String,     //File name
    pub status: String,
}

impl FileConts {
    fn new(file_name: String) -> Self {
        Self {
            buffer: vec![String::new()],
            x_pos: 0,
            y_pos: 0,
            f_name: file_name,
            status: String::from("Ctrl-S to save, Ctrl-Q to quit"),
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
    
    print!("\x1B[H\x1B[2J");

    let mut contents = FileConts::new(f_name);
    
    if let Err(why) = contents.vectorize(file_contents) {
        eprintln!("Failed to vectorize contents: {why}");
        return Ok(());
    }
    

    disable_raw_mode()?;
    /*for item in contents.buffer.clone() {
        println!("{}. {}", contents.line, item);      //In the mean time, this is just here for visualisation and testing
        contents.line += 1;
    }*/
    enable_raw_mode()?;
    
    'main: loop {
        if let Err(why) = draw::draw(contents) {
            eprintln!("Error whilst drawing contents: {}", why);
            return Ok(());
        }

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(KeyEvent { code, modifiers, .. }) => {
                    match (code, modifiers) {
                        (KeyCode::Char('x'), KeyModifiers::CONTROL) => {
                            if let Err(why) = exit::exit() {
                                contents.status = format!("Couldn't exit: {}", why);
                            }
                        }
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                            if let Err(why) = save::save(contents) {
                                contents.status = format!("Failed to save contents: {}", why);
                            }
                        }
                        (KeyCode::Char(input), KeyModifiers::NONE) => {
                            if let Err(why) = insert::insert_char(contents, input) {
                                contents.status = format!("Failed to insert char: {}", why);
                            }
                        }
                        (KeyCode::Char(input), KeyModifiers::SHIFT) => {
                            if let Err(why) = insert::insert_char(contents, input) {
                                contents.status = format!("Failed to insert char: {}", why);
                            }
                        }
                        (KeyCode::Enter, _) => {
                            if let Err(why) = insert::insert_line(contents) {
                                contents.status = format!("Failed to insert newline: {}", why);
                            }
                        }
                        (KeyCode::Backspace, _) => {
                            if let Err(why) = insert::backspace(contents) {
                                contents.status = format!("Failed to backspace: {}", why);
                            }
                        }
                        (KeyCode::Delete, _) => {
                            if let Err(why) = insert::delete(contents) {
                                contents.status = format!("Failed to delete: {}", why);
                            }
                        }
                        (KeyCode::Left, _) => {
                            if let Err(why) = cursor::move_left(contents) {
                                contents.status = format!("Failed to move left: {}", why);
                            }
                        }
                        (KeyCode::Right, _) => {
                            if let Err(why) = cursor::move_right(contents) {
                                contents.status = format!("Failed to move right: {}", why);
                            }
                        }
                        (KeyCode::Up, _) => {
                            if let Err(why) = cursor::move_up(contents) {
                                contents.status = format!("Failed to move up: {}", why);
                            }
                        }
                        (KeyCode::Down, _) => {
                            if let Err(why) = cursor::move_down(contents) {
                                contents.status = format!("Failed to move down: {}", why);
                            }
                        }
                        _ => {}
                    }

                }
            }
        }
    }

    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;

    Ok(())
}
