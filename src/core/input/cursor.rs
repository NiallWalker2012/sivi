use crate::core::input::input::FileConts;

pub fn move_left(contents: &mut FileConts, g_len: Vec<usize>) {
    if contents.x_pos > g_len[contents.y_pos] {
        contents.x_pos -= 1;
    }
}

pub fn move_right(contents: &mut FileConts, g_len: Vec<usize>) {
    if contents.x_pos < contents.buffer[contents.y_pos].len() + g_len[contents.y_pos] {
        contents.x_pos += 1;
    }
}

pub fn move_up(contents: &mut FileConts, g_len: Vec<usize>) {
    if contents.y_pos > 0 {
        contents.y_pos -= 1;

        // Clamp x_pos to new line length
        let max = contents.buffer[contents.y_pos].len() + g_len[contents.y_pos];
        if contents.x_pos > max {
            contents.x_pos = max;
        }
    }
}

pub fn move_down(contents: &mut FileConts, g_len: Vec<usize>) {
    if contents.y_pos + 1 < contents.buffer.len() {
        contents.y_pos += 1;

        // Clamp x_pos to new line length
        let max = contents.buffer[contents.y_pos].len() + g_len[contents.y_pos];
        if contents.x_pos > max {
            contents.x_pos = max;
        }
    }
}

