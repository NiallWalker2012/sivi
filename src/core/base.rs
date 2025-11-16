/* This is the base of the core directory 
 * It acts like the main.rs of it 
 */
use crate::core::load;
use crate::core::input::input;

use std::io::{
    Result,
};
use std::path::PathBuf;

pub fn start(target_path: PathBuf, file_name: String) -> Result<()> {
    let contents = match load::load_file(target_path) {
        Ok(contents) => contents,
        Err(why) => {
            eprintln!("Could not get contents: {why}");
            return Ok(());
        }
    };
    if let Err(why) = input::get_input(contents, file_name) {
        println!("Could not get user input: {why}");
        return Ok(());
    }
    Ok(())
}
