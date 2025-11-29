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
    terminal::{
        self,
        disable_raw_mode,
        enable_raw_mode,
        LeaveAlternateScreen,
    },
};
use crate::core::input::{
    draw,
    save,
    insert,
    cursor::{
        move_right,
        move_up,
        move_down,
        move_left,
    },
};

use std::path::PathBuf;

use std::io::{
    Result,
    stdout
};
use std::time::Duration;

pub struct FileConts {
    pub buffer: Vec<String>,
    pub x_pos: usize,
    pub y_pos: usize,
    pub f_name: PathBuf,     //File name
    pub status: String,
}

impl FileConts {
    fn new(file_name: PathBuf) -> Self {
        Self {
            buffer: vec![String::new()],
            x_pos: 6,
            y_pos: 0,
            f_name: file_name,
            status: String::from("Ctrl+s to save, Ctrl+q to quit"),
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

pub fn get_input(file_contents: String, f_name: PathBuf) -> Result<()> {

    execute!(stdout(), cursor::Show)?;
    
    print!("\x1B[H\x1B[2J");

    let mut contents = FileConts::new(f_name);
    
    if let Err(why) = contents.vectorize(file_contents) {
        eprintln!("Failed to vectorize contents: {why}");
        return Ok(());
    }
    
    enable_raw_mode()?;

    // I'd suggest that you skip this next bit...
    
    'main: loop {
        if let Err(why) = draw::draw(&mut contents) {
            eprintln!("Error whilst drawing contents: {}", why);
            return Ok(());
        }

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(KeyEvent { code, modifiers, .. }) => {
                    match (code, modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                            print!("\x1B[H\x1B[2J");
                            break 'main;
                        }
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                            if let Err(why) = save::save(&mut contents) {
                                contents.status = format!("Failed to save contents: {}", why);
                            }
                        }
                        (KeyCode::Char(input), KeyModifiers::NONE) => {
                            if let Err(why) = insert::insert_char(&mut contents, input) {
                                contents.status = format!("Failed to insert char: {}", why);
                            }
                        }
                        (KeyCode::Char(input), KeyModifiers::SHIFT) => {
                            if let Err(why) = insert::insert_char(&mut contents, input) {
                                contents.status = format!("Failed to insert char: {}", why);
                            }
                        }
                        (KeyCode::Enter, _) => {
                            if let Err(why) = insert::insert_line(&mut contents) {
                                contents.status = format!("Failed to insert newline: {}", why);
                            }
                        }
                        (KeyCode::Backspace, _) => {
                            if let Err(why) = insert::backspace(&mut contents) {
                                contents.status = format!("Failed to backspace: {}", why);
                            }
                        }
                        (KeyCode::Delete, _) => {
                            if let Err(why) = insert::delete(&mut contents) {
                                contents.status = format!("Failed to delete: {}", why);
                            }
                        }
                        (KeyCode::Left, _) => {
                            if let Err(why) = move_left(&mut contents) {
                                contents.status = format!("Failed to move left: {}", why);
                            }
                        }
                        (KeyCode::Right, _) => {
                            if let Err(why) = move_right(&mut contents) {
                                contents.status = format!("Failed to move right: {}", why);
                            }
                        }
                        (KeyCode::Up, _) => {
                            if let Err(why) = move_up(&mut contents) {
                                contents.status = format!("Failed to move up: {}", why);
                            }
                        }
                        (KeyCode::Down, _) => {
                            if let Err(why) = move_down(&mut contents) {
                                contents.status = format!("Failed to move down: {}", why);
                            }
                        }
                        _ => {}
                    }

                }
                _ => {}
            }
        }
    }

    // Now you see why I said that

    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;

    Ok(())
}
