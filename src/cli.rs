use std::{path::PathBuf};

use clap::Parser;

use crate::{clip::Clip, session::Session};


#[derive(Parser,Debug)]
#[command(version)] // Could add 'about' if I write something in Cargo.toml
struct Arguments {
    #[arg(long,default_value_t=false)]
    overwrite : bool, // Whether or not to overwrite the output file, if it exists
    #[arg(long)]
    start : String, // Clip start time
    #[arg(long)]
    end : String, // Clip end time
    #[arg()]
    input_file : PathBuf, // Input file
    #[arg(long)]
    label : Option<String>, // Label for the clip
    #[arg(long)]
    output_path : PathBuf, // Output directory
}

pub fn run() -> Result<(),Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    // Make sure input exists
    if !args.input_file.is_file() {
        return Err("Gotta have an input file".into());
        // TODO
    }

    // Make sure output dir exists
    if !args.output_path.is_dir() {
        return Err("gotta have valid output dir".into());
        // TODO
    }

    // Check if we should overwrite
    if args.output_path.is_file() && !args.overwrite {
        // TODO ?
    }

    let start : u32 = args.start.parse()?;
    let end : u32 = args.end.parse()?;
    // let label : &str = args.label.unwrap().clone();

    let clip : Clip = Clip::new(start, end, args.label)?;
    let file_ext : &str = match args.input_file.extension() {
        None => return Err("For now, we need file extension - we aren't letting ffmpeg dynamically discover it".into()),
        Some(ext) => match ext.to_str() {
            Some(s) => s,
            None => return Err("File extension must be valid UTF-8, no funny business".into())
        }
    };

    let mut vec = Vec::new();
    vec.push(clip);
    let session = Session::new(Some(args.input_file), Some(args.output_path), vec, None, false);
    return session.extract_clips(args.overwrite);

}
