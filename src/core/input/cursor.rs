use std::io::Result;
use crate::core::input::input::FileConts;

pub fn move_left(contents: &mut FileConts) -> Result<()> {
    if contents.x_pos > 6 {
        contents.x_pos -= 1;
    }
    Ok(())
}

pub fn move_right(contents: &mut FileConts) -> Result<()> {
    if contents.x_pos < contents.buffer[contents.y_pos].len() + 6 {
        contents.x_pos += 1;
    }
    Ok(())
}

pub fn move_up(contents: &mut FileConts) -> Result<()> {
    if contents.y_pos > 0 {
        contents.y_pos -= 1;
    }
    Ok(())
}

pub fn move_down(contents: &mut FileConts) -> Result<()> {
    if contents.y_pos + 1 < contents.buffer.len() {
        contents.y_pos += 1;
    } 
    Ok(())
}
