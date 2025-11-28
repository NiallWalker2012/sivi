/*This is the main.rs file
 * It acts like a backbone of linkage, as it calls all of
 * the external sections/files 
 */

use std::env;
use std::path::{
    Path,
    PathBuf,
};
use std::io::{
    Result,
    stdout,
};
use crossterm::{
    execute,
    cursor,
    terminal::disable_raw_mode,
};
use std::fs::File;

mod args;
mod explorer;
mod core;

fn main() -> Result<()> {
    //Go to parent directory - the command line argument should be in that directory
    env::set_current_dir("..")?;

    let args = args::parse();       //Get the command line args from the external args from args.rs
    let target_name = args.target;

    let targ_check_path = Path::new(&target_name);
    let targ_path: Option<PathBuf>;

    if !targ_check_path.exists() {
        File::create(targ_check_path)?;
    }

    if targ_check_path.is_dir() {       //If the argument is a folder, go to file explorer
        env::set_current_dir(&target_name)?;
        targ_path = match explorer::main() {            //Open file explorer from explorer.rs 
        Err(why) => {       //Exit if a function fails in explorer.rs
            eprintln!("Failed to get target: {why}");
            return Ok(());
        }
        Ok(None) => {       //Exit if no file is selected in explorer.rs 
            return Ok(());
        }
        Ok(Some(value)) => Some(value),     /*Return the value to targ_path if all is
                                                * successful*/
    };
    print!("\x1B[H\x1B[2J");            //ANSI escape code: clear terminal
    } else {
        targ_path = Some(PathBuf::from(target_name.clone()));             //Otherwise, continue as normal
    }
        
    //Start the editor
    if let Err(why) = core::base::start(targ_path.unwrap()) {
        eprintln!("Could not start editor: {why}");
        let _ = cleanup()?;
    }

    //Re-do cleanup to ensure a smooth exit 
    let _ = cleanup()?;

    Ok(())
}

fn cleanup() -> Result<()> {
    execute!(stdout(), cursor::Show)?;
    disable_raw_mode()?;

    Ok(())
}
