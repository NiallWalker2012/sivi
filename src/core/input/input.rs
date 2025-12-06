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
        disable_raw_mode,
        enable_raw_mode,
        LeaveAlternateScreen,
        EnterAlternateScreen,
        self,
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

#[derive(Default)]
pub struct FileConts {
    pub buffer: Vec<String>,
    pub x_pos: usize,
    pub y_pos: usize,
    pub top_bord: usize,
    pub f_name: PathBuf,     //File name
    pub status: String,
}

impl FileConts {
    fn new(file_name: PathBuf) -> Self {
        Self {
            buffer: vec![String::new()],
            // To allow line numbering
            x_pos: 8,
            y_pos: 0,
            top_bord: 0,
            f_name: file_name,
            status: "Ctrl+s to save, Ctrl+q to quit".to_string(),
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

    execute!(stdout(), EnterAlternateScreen, cursor::Show)?;

    let (_, text_height) = terminal::size()?;
    text_height.saturating_sub(1) as usize;

    let mut contents = FileConts::new(f_name);
    
    if let Err(why) = contents.vectorize(file_contents) {
        eprintln!("Failed to vectorize contents: {why}");
        return Ok(());
    }

    enable_raw_mode()?;

    // This next bit is very ugly...
    
    'main: loop {
        match draw::draw(&mut contents) {
            Err(why) => {
                eprintln!("Error whilst drawing contents: {}", why);
                return Ok(());
            }
            Ok(_) => {}
        };
        
        if event::poll(Duration::from_millis(50))? {
            // Reads the raw user input
            match event::read()? {
                Event::Key(KeyEvent { code, modifiers, .. }) => {
                    // Compares the key pressed and the modifier (e.g. Ctrl) and selectes the
                    // corresponding action
                    match (code, modifiers) {
                        // If Ctrl + q is pressed, exit
                        (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                            // Breaks from the main loop and exits
                            break 'main;
                        }
                        // If Ctrl + s is pressed, save
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                            if let Err(why) = save::save(&mut contents) {
                                contents.status = format!("Failed to save contents: {}", why);
                            }
                        }
                        // If any character is pressed, with either no or a shift modifier, insert
                        (KeyCode::Char(input), KeyModifiers::NONE) => {
                            insert::insert_char(&mut contents, input);
                        }
                        (KeyCode::Char(input), KeyModifiers::SHIFT) => {
                            insert::insert_char(&mut contents, input);
                        }
                        // If enter is pressed
                        (KeyCode::Enter, _) => {
                            insert::insert_line(&mut contents);
                        }
                        // If backspace is pressed
                        (KeyCode::Backspace, _) | (KeyCode::Char('\u{7f}'), _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                            insert::backspace(&mut contents); 
                        }
                        // If tab is pressed
                        (KeyCode::Tab, _) => {
                            for _i in 0..4 {
                                insert::insert_char(&mut contents, ' ');
                            }
                        }
                        // If delete is pressed
                        (KeyCode::Delete, _) => {
                            insert::delete(&mut contents);
                        }
                        // If arrow keys are pressed
                        (KeyCode::Left, _) => {
                            move_left(&mut contents);
                        }
                        (KeyCode::Right, _) => {
                            move_right(&mut contents);
                        }
                        (KeyCode::Up, _) => {
                            move_up(&mut contents);
                        }
                        (KeyCode::Down, _) => {
                            move_down(&mut contents, text_height.into()); 
                        }
                        _ => {}
                    }

                }
                _ => {}
            }
        }
    }


    execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;

    Ok(())
}
