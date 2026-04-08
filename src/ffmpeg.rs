use crate::clip::Clip;
use std::path::PathBuf;
use std::process::Command;

// let args = &[
//     "-ss", "00:00:20",
//     "-to", "00:00:30",
//     "-i", "/home/peter/Downloads/test.mp4",
//     "-c", "copy",
//     "-avoid_negative_ts", "make_zero",
//     "/home/peter/Videos/output.mp4"
// ];

fn get_commandline_args(clip: &Clip, source: &PathBuf, output: &PathBuf) -> Vec<String> {
    let mut vec : Vec<String> = Vec::new();
    vec.push("-ss".to_string());
    vec.push(get_timestring_from_seconds(clip.start));
    vec.push("-to".to_string());
    vec.push(get_timestring_from_seconds(clip.end));
    vec.push("-i".to_string());
    vec.push(source.to_string_lossy().into_owned());
    vec.push("-c".to_string());
    vec.push("copy".to_string());
    vec.push("-avoid_negative_ts".to_string());
    vec.push("make_zero".to_string());
    vec.push(output.to_string_lossy().to_string());
    return vec;
}

fn get_timestring_from_seconds(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, secs);
}

pub fn extract_clip(clip: &Clip, source: &PathBuf, output: &PathBuf) -> Result<(), String> {
    let vec = get_commandline_args(clip, source, output);
    println!("In extract_clip - I should do something here");
    return Err(format!("ffmpeg {}",vec.join(" ")));
}

