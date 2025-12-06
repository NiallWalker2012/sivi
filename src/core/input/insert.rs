use crate::core::input::{
    input::FileConts, 
};
use std::mem::take;

use std::process::exit;

use std::io::Result;



fn get_height() -> Result<usize> {
     // Get the text height
    let (_, height) = crossterm::terminal::size()?;  
    let text_height = height.saturating_sub(1); // leave last line for status
    
    return Ok(text_height.into());
}


pub fn insert_char(contents: &mut FileConts, input: char) {
    if contents.y_pos >= contents.buffer.len() {
        contents.buffer.push(String::new());
    }

    let gutter = 8;
    let line = &mut contents.buffer[contents.y_pos];

    // Convert display x_pos to character position in the buffer
    let char_pos = contents.x_pos.saturating_sub(gutter);

    // Convert character index to byte index
    let byte_pos = line
        .char_indices()
        .nth(char_pos)
        .map(|(i, _)| i)
        .unwrap_or_else(|| line.len());

    line.insert(byte_pos, input);

    // Move visual cursor forward one display column
    contents.x_pos += 1;
}


pub fn insert_line(contents: &mut FileConts) {
    // If cursor is beyond the last line, add a new empty line
    if contents.y_pos >= contents.buffer.len() {
        contents.buffer.push(String::new());
        contents.x_pos = 8; // start after gutter
        contents.y_pos += 1;
        return;
    }

    // Take ownership of the current line to safely mutate buffer
    let mut line = take(&mut contents.buffer[contents.y_pos]);

    // Convert x_pos (char index) to byte index
    let idx = contents.x_pos.saturating_sub(8);
        
    // Split line at cursor
    let new_line = line.split_off(idx);

    // Put the original line back
    contents.buffer[contents.y_pos] = line;

    // Move cursor to the new line
    contents.y_pos += 1;
    contents.x_pos = 8; // start after gutter

    // Insert the new line into the buffer
    contents.buffer.insert(contents.y_pos, new_line);

    let text_height: usize = match get_height() {
        Err(_) => {
            exit(1);
        }
        Ok(val) => val,
    };
    // Check if it is necessary to scroll down, to prevent terminal size panics
    if contents.y_pos >= contents.top_bord + text_height - 5 {
        contents.top_bord = contents.y_pos - text_height + 1;
    }

}



pub fn backspace(contents: &mut FileConts) {
    // If at very beginning, nothing to delete
    if contents.x_pos == 8 && contents.y_pos == 0 {
        return;
    }

    if contents.x_pos > 8 {
        // Backspace within the current line
        let line = &mut contents.buffer[contents.y_pos];

        let idx = contents.x_pos - 8; // convert to buffer index

        if idx > 0 && idx <= line.len() {
            line.remove(idx - 1);
            contents.x_pos -= 1;
        }
    } else {
        // Backspace at the line start: merge with previous line
        let removed = contents.buffer.remove(contents.y_pos);
        contents.y_pos -= 1;

        let prev = &mut contents.buffer[contents.y_pos];
        let old_len = prev.len();

        prev.push_str(&removed);
        contents.x_pos = old_len + 8;
        

        // Check if it is necessary to scroll up, to prevent terminal size panics
        if contents.y_pos < contents.top_bord + 4 {
            contents.top_bord = contents.y_pos;
        }
    }
}

pub fn delete(contents: &mut FileConts) {
    // Create variable for buffer's y axis
    let line = &mut contents.buffer[contents.y_pos];
    
    let idx = contents.x_pos - 8;

    if idx >= line.len() {
        return;
    }
    line.remove(idx);
}
