use std::path::PathBuf;

use clap::Parser;

use crate::clip::Clip;


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

    let start : u32 = args.start.parse().expect("start needs to be an integer for now");
    let end : u32 = args.end.parse().expect("end needs to be an integer for now");
    // let label : &str = args.label.unwrap().clone();

    let clip : Clip = Clip::new(start, end, args.label)?;
    let file_ext : &str = args.input_file.extension().unwrap().to_str().unwrap();

    let output_file_name = format!("{}_{}-{}.{}", clip.label(), start, end, file_ext);
    let mut output_file = PathBuf::from(args.output_path);
    output_file.push(output_file_name);

    return crate::ffmpeg::extract_clip(&clip, &args.input_file, &output_file, args.overwrite);
}
