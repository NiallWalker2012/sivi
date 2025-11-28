use crossterm::{
    cursor,
    style::{
        Print,
        Attribute,
        SetAttribute,
    },
    terminal::{
        self,
        ClearType,
    },
    queue,
};

use std::io::{
    Result,
    stdout,
    Write,
};

use crate::core::input::input::FileConts;

pub fn draw(conts: &mut FileConts) -> Result<()> {
    let mut stdout = stdout();

    let (width, height) = terminal::size()?;
    let height = height as usize;
    // leave last line for status
    let text_height = if height > 1 { height - 1 } else { 0 };

    queue!(stdout, terminal::Clear(ClearType::All))?;

    for (i, line) in conts.buffer.iter().take(text_height).enumerate() {
        queue!(
            stdout,
            cursor::MoveTo(0, i as u16),
            Print(i + 1),
            Print(".    "),
            Print(if line.len() > width as usize {
                // trim to terminal width
                let mut s = line.clone();
                s.truncate(width as usize);
                s
            } else {
                line.clone()
            })
        )?;
    }

    // status bar on last line
    let total_chars: usize = conts.buffer.iter().map(|l| l.len()).sum();
    let file_name = &conts.f_name;
    let mut status_full = format!("{} — {} chars | {}", file_name.display(), total_chars, conts.status);

    if status_full.len() > width as usize {
        status_full.truncate(width as usize);
    }


    // draw inverse style status
    queue!(
        stdout,
        cursor::MoveTo(0, text_height as u16),
        SetAttribute(Attribute::Reverse),
        Print(&status_full),
        SetAttribute(Attribute::Reset)
    )?;

    // move cursor
    let cx = conts.x_pos as u16;
    let cy = conts.y_pos as u16;
    queue!(stdout, cursor::MoveTo(cx, cy))?;

    stdout.flush()?;
    Ok(())
}

