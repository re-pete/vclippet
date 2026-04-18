#![allow(clippy::needless_return)]
#![allow(clippy::vec_init_then_push)]
// #![allow(clippy::needless_late_init)]
#![allow(unused_variables)]
// #![allow(dead_code)]
#![allow(warnings)]

mod cli;
mod clip;
mod ffmpeg;
mod session;
mod tui;

fn main() {
    // If no args, launch TUI mode
    if std::env::args().count() == 1 {
        let _ = tui::run();
        return;
    } else {
        match crate::cli::run() {
            Ok(()) => println!("We are in main, cli.run returned OK"),
            Err(str) => println!("We are in main, cli.run returned an error: {}", str),
        }
    }
}
