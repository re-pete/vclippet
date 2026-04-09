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

use clip::Clip;
use std::path::PathBuf;

fn main() {
    let mut clip: Clip = Clip::new(10,30,None).unwrap();
    clip.label = Some("clipname".to_string());

    let mut source : PathBuf = PathBuf::new();
    source.push("./test_input.mp4");
    let mut output : PathBuf = PathBuf::new();
    output.push("./test_output.mp4");

    let ffmpeg = ffmpeg::extract_clip(&clip,&source,&output);
    match ffmpeg  {
        Ok(()) => {
            println!("This shouldn't happen right now");
        }
        Err(str) => {
            println!("{}",str);
        }
    }
 
}
