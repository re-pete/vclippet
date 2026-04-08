use crate::clip::Clip;
use std::path::PathBuf;
use std::process::Command;

const CHECK_IF_FILE_IS_VALID : &str = "ffmpeg -v error -sseof -60 -i {} -f null -";
const EXTRACT_CLIP : &str = "-ss {} -to {} -i {} -c copy -avoid_negative_ts make_zero {}";

fn get_commandline_args(clip: &Clip, source_file: &PathBuf, output_file: &PathBuf) -> Vec<String> {
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

pub fn extract_clip(clip: &Clip, source_file: &PathBuf, output_file: &PathBuf) -> Result<(), String> {

    // Make sure input exists
    if source_file.is_file() != true {
        return Err("source_file file doesn't exist".to_string());
    }

    // Make sure output_file directory exists
    // Rust sucks, so verbose, because we can't use ? here
    let output_parent_dir_option = output_file.parent();
    let output_parent_dir : PathBuf;
    match output_parent_dir_option {
        Some(some) => {
            output_parent_dir = some.to_path_buf();
        }
        None => {
            return Err("You must have passed in the root directory instead of a file".to_string());
        }
    }

    if output_parent_dir.is_dir() != true {
        return Err("Output directory must exist".to_string());
    }
    
    if output_file.is_file() == true {
        return Err("We'll handle file collisions later".to_string());
    }


    let vec = get_commandline_args(clip, source_file, output_file);
    let cmd_output = Command::new("ffmpeg").args(&vec).output().unwrap();
    // At this point it runs, should figure out how to get the output of the command
    // to the console for now
    println!("ffmpeg command status: {}", cmd_output.status);
    return Err(format!("ffmpeg {}",vec.join(" ")));
}

