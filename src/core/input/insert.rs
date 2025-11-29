use crate::core::input::input::FileConts;

pub fn insert_char(contents: &mut FileConts, input: char) {
    if contents.y_pos >= contents.buffer.len() {
        contents.buffer.push(String::new());
    }
    let line = &mut contents.buffer[contents.y_pos];
    if contents.x_pos <= line.len() {
        //Subtracted by 6 to account for line numbering
        line.insert(contents.x_pos - 6, input);
    } else {
        line.push(input);
    }
    contents.x_pos += 1;
}

pub fn insert_line(contents: &mut FileConts) {
    if contents.y_pos >= contents.buffer.len() {
        contents.buffer.push(String::new());
        // Adjust cursor positioning
        contents.x_pos = 6;
        contents.y_pos += 1;
        return;
    }
    let line = &mut contents.buffer[contents.y_pos];
    // Slice previous line by x position
    let mut new_line = String::new();
    if contents.x_pos <= line.len() {
        new_line = line.split_off(contents.x_pos);
    }
    contents.y_pos += 1;
    contents.x_pos = 6;

    if contents.x_pos <= line.len() {
        // Insert the new line to buffer variable
        contents.buffer.insert(contents.y_pos, new_line);
    } else {
        contents.buffer.push(new_line);
    }
}

pub fn backspace(contents: &mut FileConts) {
    // If at very beginning, nothing to delete
    if contents.x_pos == 6 && contents.y_pos == 0 {
        return;
    }

    if contents.x_pos > 6 {
        // Backspace within the current line
        let line = &mut contents.buffer[contents.y_pos];

        let idx = contents.x_pos - 6; // convert to buffer index

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
        contents.x_pos = old_len + 6;
    }

}

pub fn delete(contents: &mut FileConts) {
    let line = &mut contents.buffer[contents.y_pos];
    
    let idx = contents.x_pos - 6;

    if idx >= line.len() {
        return;
    }
    line.remove(idx);
}
