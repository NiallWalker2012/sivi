use crossterm::{
    cursor,
    style::{
        Print,
        Attribute,
        SetAttribute,
        Stylize,
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

fn make_gutter(line_num: usize) -> String {
    let line_size = line_num.to_string().len();

    let space_amount = 8 - line_size;
    let mut gutter_conts: Vec<String> = vec![];

    gutter_conts.push(line_num.to_string());

    for _i in 0..space_amount {
        gutter_conts.push(" ".to_string());
    }
    return gutter_conts.join("").bold().yellow().to_string();
}

pub fn draw(conts: &mut FileConts) -> Result<Vec<usize>> {
    let mut stdout = stdout();

    let (width, height) = terminal::size()?;
    let height = height as usize;
    // leave last line for status
    let text_height = if height > 1 { height - 1 } else { 0 };

    queue!(stdout, terminal::Clear(ClearType::All))?;

    let mut gutter_len: Vec<usize> = vec![];

    for (i, line) in conts.buffer.iter().take(text_height).enumerate() {
        let line_numberer = make_gutter(i + 1); 
        queue!(
            stdout,
            cursor::MoveTo(0, i as u16),
            Print(line_numberer),
            Print(if line.len() > width as usize {
                // trim to terminal width
                let mut s = line.clone();
                s.truncate(width as usize);
                s
            } else {
                line.clone()
            })
        )?;
        gutter_len.push(8);
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
    return Ok(gutter_len);
}

