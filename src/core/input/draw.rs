/* Draw.rs:
*   Recieves the file contents as an arguments, and draws them,
*   leaving a space at the bottom for the status bar, which contains
*   the save and quit shortcut and the cursor's coordinates.
*   Also, it draws the gutter containing the line numbering
*/

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
    },
    queue,
};

use std::io::{
    Result,
    stdout,
    Write,
};
use std::fs::read_to_string;

use crate::core::input::input::FileConts;

/* Function that makes the gutter (which contains the line number) a constant width,
 * no matter how many characters in the number. If a file contains more than 1E + 8,
 * the program will panic as the gutter width is not large enough. As of right now, iter
 * am not too bothered about fixing it as there are much bigger problems
 */
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


pub fn draw(conts: &mut FileConts) -> Result<()> {
    let mut stdout = stdout();

    let (width, height) = terminal::size()?;
    let text_height = height.saturating_sub(1) as usize;

    let buf_len = conts.buffer.len();
    let top = conts.top_bord.min(buf_len);
    let bottom = (top + text_height).min(buf_len);


    for (i, line) in conts.buffer[top..bottom].iter().enumerate() {
        let line_number = top + i + 1;
        let gutter = make_gutter(line_number);

        queue!(
            stdout,
            cursor::MoveTo(0, i as u16),
            Print(gutter),
            Print(line)
        )?;
    }


    // status bar on last line
    let total_chars: usize = conts.buffer.iter().map(|l| l.len()).sum();
    let file_name = &conts.f_name;
    // Check for unsaved changes:
    let fileconts = read_to_string(file_name)?;
    if fileconts != conts.buffer.join("\n") {
        conts.status = format!("You have unsaved changes");
    }

    let mut status_full = format!("{} — {} chars | {}       {}:{}", file_name.display(), total_chars, conts.status, conts.y_pos + 1, conts.x_pos - 7);

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
    let cy = (conts.y_pos - conts.top_bord) as u16;
    queue!(stdout, cursor::MoveTo(cx, cy))?;

    stdout.flush()?;
    return Ok(());
}

