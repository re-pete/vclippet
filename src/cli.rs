use std::path::PathBuf;

use clap::Parser;

use crate::{clip::Clip, session::Session};

#[derive(Parser, Debug)]
#[command(version)] // Could add 'about' if I write something in Cargo.toml
struct Arguments {
    /// force overwriting of an existing file
    #[arg(long, default_value_t = false)]
    overwrite: bool,
    /// Clip start time
    #[arg(long)]
    start: String,
    /// Clip end time
    #[arg(long)]
    end: String,
    /// Input file
    #[arg()]
    input_file: PathBuf,
    /// Label for the clip
    #[arg(long)]
    label: Option<String>,
    /// Output directory
    #[arg(long)]
    output_path: PathBuf,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
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

    let start: u32 = args.start.parse()?;
    let end: u32 = args.end.parse()?;
    let clip: Clip = Clip::new(start, end, args.label)?;

    let mut session = Session::default();
    session.add_clip(clip);
    session.set_source_file(args.input_file)?;
    session.output_path = Some(args.output_path);
    return session.extract_clips(args.overwrite);
}
