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
    vec.push("-y".to_string());
    vec.push(output_file.to_string_lossy().to_string());
    return vec;
}

fn get_timestring_from_seconds(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, secs);
}

// Ok, changing responsibility of this function.
// This only accepts a Clip, an input file path, and an output file path. 
// It does nothing but run ffmpeg on exactly its inputs.  No logic allowed here
pub fn extract_clip(clip: &Clip, source_file: &Path, output_file: &Path, overwrite: bool) -> Result<(), String> {

    // Make sure input exists
    if !source_file.is_file() {
        return Err("source_file file doesn't exist".to_string());
    }

    if output_file.is_file() && !overwrite {
        return Err("File exists, use --overwrite to overwrite".to_string());
    }

    let vec = get_commandline_args(clip, source_file, output_file);
    let cmd_output = Command::new("ffmpeg").args(&vec).output().unwrap();
    println!("ffmpeg command stdout: {}", String::from_utf8_lossy(&cmd_output.stdout));
    println!("ffmpeg command stderr: {}", String::from_utf8_lossy(&cmd_output.stderr));
    println!("Ran with the following command: ffmpeg {}",vec.join(" "));
    return Ok(());
}

