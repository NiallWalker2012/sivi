use crate::core::input::input::FileConts;


pub fn scroll_down(conts: &mut FileConts, text_height: usize) {
    if conts.y_pos + 1 < conts.buffer.len() {
        conts.y_pos += 1;

        if conts.y_pos >= conts.top_bord + text_height {
            conts.top_bord = conts.y_pos - text_height + 1;
        }
    }
}

pub fn scroll_up(conts: &mut FileConts) {
    if conts.y_pos > 0 {
        conts.y_pos -= 1;
        if conts.y_pos < conts.top_bord {
            conts.top_bord = conts.y_pos;
        }
    }
}

