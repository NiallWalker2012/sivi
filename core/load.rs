use std::path::PathBuf;
use std::fs::File;
use std::io::{
    Read,
    Result,
};

pub fn load_file(file_path: PathBuf) -> Result<()> {
    //Clear screen before doing anything
    print!("\x1B[H\x1B[2J");
    //Open file for reading
    let file = File::open(&file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    Ok(())
}
