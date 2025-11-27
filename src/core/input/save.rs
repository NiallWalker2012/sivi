use std::fs::{
    self,
};
use std::io::Result;
use crate::core::input::input::FileConts;

pub fn save(contents: &mut FileConts) -> Result<()> {
    let str_conts: String = contents.buffer.join("\n");
    fs::write(contents.f_name.clone(), str_conts)?;
    contents.status = format!("Successfully saved to {}", contents.f_name);
    
    Ok(())
}
