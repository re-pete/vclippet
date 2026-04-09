use crate::clip::Clip;
use std::path::Path;
use std::process::Command;

const CHECK_IF_FILE_IS_VALID : &str = "ffmpeg -v error -sseof -60 -i {} -f null -";
const EXTRACT_CLIP : &str = "-ss {} -to {} -i {} -c copy -avoid_negative_ts make_zero {}";

fn get_commandline_args(clip: &Clip, source_file: &Path, output_file: &Path) -> Vec<String> {
    let mut vec : Vec<String> = Vec::new();
    vec.push("-ss".to_string());
    vec.push(get_timestring_from_seconds(clip.start));
    vec.push("-to".to_string());
    vec.push(get_timestring_from_seconds(clip.end));
    vec.push("-i".to_string());
    vec.push(source_file.to_string_lossy().into_owned());
    vec.push("-c".to_string());
    vec.push("copy".to_string());
    vec.push("-avoid_negative_ts".to_string());
    vec.push("make_zero".to_string());
    vec.push(output_file.to_string_lossy().to_string());
    return vec;
}

fn get_timestring_from_seconds(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, secs);
}

fn get_filename(clip: &Clip) -> &str {
    // Implement this
    return "";
}

pub fn extract_clip(clip: &Clip, source_file: &Path, output_directory: &Path) -> Result<(), String> {

    // Make sure input exists
    if !source_file.is_file() {
        return Err("source_file file doesn't exist".to_string());
    }

    println!("{}",clip.label());
    // Make sure output_directory directory exists
    if !output_directory.is_dir() {
        return Err("Output directory must exist".to_string());
    }

    // Check if output file exists
    // let output_file = output_directory + get_filename(&clip);
 
    if output_directory.is_file() {
        return Err("We'll handle file collisions later".to_string());
    }


    let vec = get_commandline_args(clip, source_file, output_directory);
    let cmd_output = Command::new("ffmpeg").args(&vec).output().unwrap();
    // At this point it runs, should figure out how to get the output of the command
    // to the console for now
    println!("ffmpeg command status: {}", cmd_output.status);
    return Err(format!("ffmpeg {}",vec.join(" ")));
}

