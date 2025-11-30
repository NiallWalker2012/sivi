use std::fs::{
    self,
};
use std::io::Result;
use crate::core::input::input::FileConts;

pub fn save(contents: &mut FileConts) -> Result<()> {
    contents.status = format!("Saving...");
    let str_conts: String = contents.buffer.join("\n");
    fs::write(contents.f_name.clone(), str_conts)?;
    contents.status = format!("Successfully saved!");
    
    Ok(())
}
