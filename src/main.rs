#![allow(clippy::needless_return)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::needless_late_init)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod cli;
mod tui;
mod clip;
mod ffmpeg;
mod session;

fn main() {
    // If no args, launch TUI mode
    if std::env::args().count() == 1 {
        println!("This is where I would launch a TUI");
        println!("IF I HAD ONE!");
        return;
    } else {
        match crate::cli::run() {
            Ok(()) => println!("We are in main, cli.run returned OK"),
            Err(str) => println!("We are in main, cli.run returned an error: {}",str)
        }
    }
}
