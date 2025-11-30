use crate::core::input::input::FileConts;
use std::mem::take;

pub fn insert_char(contents: &mut FileConts, input: char, g_len: Vec<usize>) {
    if contents.y_pos >= contents.buffer.len() {
        contents.buffer.push(String::new());
    }
    let line = &mut contents.buffer[contents.y_pos];
    if contents.x_pos <= line.len() {
        //Subtracted by 6 to account for line numbering
        line.insert(contents.x_pos - g_len[contents.y_pos], input);
    } else {
        line.push(input);
    }
    contents.x_pos += 1;
}

pub fn insert_line(contents: &mut FileConts, g_len: Vec<usize>) {
    // If cursor is beyond the last line, add a new empty line
    if contents.y_pos >= contents.buffer.len() {
        contents.buffer.push(String::new());
        contents.x_pos = g_len[contents.y_pos]; // start after gutter
        contents.y_pos += 1;
        return;
    }

    // Take ownership of the current line to safely mutate buffer
    let mut line = take(&mut contents.buffer[contents.y_pos]);

    // Convert x_pos (char index) to byte index
    let idx = contents.x_pos.saturating_sub(g_len[contents.y_pos]);
        
    // Split line at cursor
    let new_line = line.split_off(idx);

    // Put the original line back
    contents.buffer[contents.y_pos] = line;

    // Move cursor to the new line
    contents.y_pos += 1;
    contents.x_pos = g_len[contents.y_pos - 1]; // start after gutter

    // Insert the new line into the buffer
    contents.buffer.insert(contents.y_pos, new_line);
}



pub fn backspace(contents: &mut FileConts, g_len: Vec<usize>) {
    // If at very beginning, nothing to delete
    if contents.x_pos == g_len[contents.y_pos] && contents.y_pos == 0 {
        return;
    }

    if contents.x_pos > g_len[contents.y_pos] {
        // Backspace within the current line
        let line = &mut contents.buffer[contents.y_pos];

        let idx = contents.x_pos - g_len[contents.y_pos]; // convert to buffer index

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
        contents.x_pos = old_len + g_len[contents.y_pos];
    }

}

pub fn delete(contents: &mut FileConts, g_len: Vec<usize>) {
    // Create variable for buffer's y axis
    let line = &mut contents.buffer[contents.y_pos];
    
    let idx = contents.x_pos - g_len[contents.y_pos];

    if idx >= line.len() {
        return;
    }
    line.remove(idx);
}
