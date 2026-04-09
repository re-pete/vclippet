#![allow(clippy::needless_return)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::bool_comparison)]
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
    println!("Clip created: {} to {} with maybe a name of: {}", clip.start, clip.end, clip.name.as_deref().unwrap_or("output"));
    clip.name = Some("clipname".to_string());
    println!("Clip created: {} to {} with maybe a name of: {}", clip.start, clip.end, clip.name.as_deref().unwrap_or("output"));

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
    
    let clip2: Clip = Clip::new(30,10,Some("this shoul cause a panic".to_string())).unwrap();
}
