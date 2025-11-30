/*This is a TUI file explorer that allows 
 * the user to select a target file if a folder,
 * not a file, is provided.
 * If a file is provided, this will not be run 
 */

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
use std::env;
use std::io::{
    Result,
    stdout,
    Write
};
use std::fs;

use std::path::PathBuf;

pub fn get_target() -> Result<Option<PathBuf>> {
    let mut selected = 0;
    let mut dir = env::current_dir()?;      //This gets the current directory (like a pwd command)

    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    loop {
        //Get all entries in current directory (like a ls/dir command)
        let mut menu_items: Vec<String> = if let Ok(entries) = fs::read_dir(&dir) {
            entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .collect()
        } else {
            vec!["Error reading directory".to_string()]
        };
        
        //Insert [Back] (go to parent dir) and [Exit] option 
        menu_items.insert(0, "[Exit]".to_string());
        if dir.parent().is_some() {
            menu_items.insert(1, "[Back]".to_string());
        }
        
        //Correct selection
        if selected >= menu_items.len() {
            selected = menu_items.len().saturating_sub(1);
        }

        //Draw TUI
        execute!(
            stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::FromCursorDown)
        )?;
        println!("{}", "Please locate the file you wish to edit".with(Color::Blue));

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout(), cursor::MoveTo(0, (i + 1) as u16))?;
            execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;

            //Determine styling and print option
            let display_item = 
            if item == "[Back]" {
                item.clone().with(Color::Green).bold().to_string()  //Style it as bold and green
            } else if item == "[Exit]" {
                item.clone().with(Color::Red).bold().to_string()    //Style it as bold and red 
            } else if dir.join(item).is_dir() {             //If selected is a folder
                item.clone().with(Color::Blue).bold().to_string()   //Style it as bold and blue
            } else {
                item.clone()
            };

            // Highlight if selected option
            if i == selected {
                print!("    {}", display_item.on_white().black());
            } else {
                print!("    {}", display_item);
            }
        }

        stdout().flush()?;      // Flush output and ensure everything is printed

        // Handle user input using crossterm raw input mode 
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < menu_items.len().saturating_sub(1) {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    let selected_item = &menu_items[selected];
                    //Handle if [Exit] or [Back] is selected
                    if selected_item == "[Exit]" {
                        execute!(stdout(), cursor::Show)?;
                        disable_raw_mode()?;
                        return Ok(None);
                    } else if selected_item == "[Back]" {
                        if let Some(parent) = dir.parent() {
                            dir = parent.to_path_buf();
                            selected = 0;
                        }
                        continue;
                    }
                    
                    // Handle if a folder or file is selected
                    let path = dir.join(selected_item);
                    if path.is_dir() {
                        dir = path;
                        selected = 0;
                        continue;
                    }
                    //If it is a file, return that path
                    return Ok(Some(path));
                }
                _ => {}
            }
        }
    }
}
