/*This is the function to get the command 
 * line argument (the pathname)
 * It is similar to doing the C code:
 *
 * int main(int argc, char **argv) 
 *
 * but for Rust. However, it uses the 
 * external library clap (Command Line Argument Parser),
 * which provides automated help features if no arguments are given
 */

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "sivi: A simple, lightweight, TUI text editor"
)]
pub struct Args {
    //Target filename or folder name
    pub target: String,
}

pub fn parse() -> Args {
    Args::parse()   //This parses the arguments
}

