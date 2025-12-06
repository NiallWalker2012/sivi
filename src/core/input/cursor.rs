use crate::core::input::{
    input::FileConts,
};

pub fn move_left(contents: &mut FileConts) {
    if contents.x_pos > 8 {
        contents.x_pos -= 1;
    }
}

pub fn move_right(contents: &mut FileConts) {
    let max = contents.buffer[contents.y_pos].len() + 8;
    if contents.x_pos < max {
        contents.x_pos += 1;
    }
}

pub fn move_up(contents: &mut FileConts) {
    if contents.y_pos == 0 {
        return;
    }

    contents.y_pos -= 1;

    let max = contents.buffer[contents.y_pos].len() + 8;
    if contents.x_pos > max {
        contents.x_pos = max;
    }

    // SCROLL
    if contents.y_pos < contents.top_bord + 4 {
        contents.top_bord = contents.y_pos;
    }
}

pub fn move_down(contents: &mut FileConts, text_height: usize) {
    if contents.y_pos + 1 >= contents.buffer.len() {
        return;
    }

    contents.y_pos += 1;

    let max = contents.buffer[contents.y_pos].len() + 8;
    if contents.x_pos > max {
        contents.x_pos = max;
    }

    // SCROLL
    if contents.y_pos >= contents.top_bord + text_height - 5 {
        contents.top_bord = contents.y_pos - text_height + 1;
    }
}

