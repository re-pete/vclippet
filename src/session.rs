use crate::clip::Clip;
use std::{error::Error, path::{Path, PathBuf}};

#[derive(Default)]
pub struct Session {
    source_file: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub clips: Vec<Clip>,
    pub session_name: Option<String>,
    pub concat: bool,
}

impl Session { 
    pub fn new(source_file: Option<PathBuf>, output_path: Option<PathBuf>, clips: Vec<Clip>, session_name: Option<String>, concat: bool) -> Session {
        Session { source_file, output_path, clips, session_name, concat }
    }

    pub fn add_clip(&mut self, clip:Clip) {
        self.clips.push(clip)
    }

    pub fn set_source_file(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        // Input file must exist
        if !path.is_file() {
            return Err("Gotta have an input file".into());
        }
        // The logic here is a little weird but - get_file_ext_from_path returns an error if there isn't a valid file extension
        // So just calling it and having it not fail is enough validation Maybe eventually we pick from a whitelist of video types
        get_file_ext_from_path(path.as_ref())?;
        // At this point, file exists and has some kind of file extension
        self.source_file = Some(path);

        return Ok(());

    }

    fn get_file_ext(&self) -> Result<String, Box<dyn Error>> {
        let source_file = self
            .source_file
            .as_ref()
            .ok_or("source file must be set before accessing file ext")?;

        get_file_ext_from_path(source_file)
    }

    fn get_output_filename(&self, clip: &Clip) -> Result<String, Box<dyn Error>> {

        // If the session name is set, it overrides individual labels
        let file_label = self.session_name.as_deref().unwrap_or_else(|| clip.label());
        

        // If we are concatenating a bunch of clips, we don't worry about timestamps
        let clip_str = if self.concat {
            String::from("")
        } else {
            format!("_{}-{}", clip.start, clip.end)
        };
        let ext = self.get_file_ext()?;
        return Ok(format!("{}{}.{}",file_label,clip_str,ext));
    }
    
    pub fn extract_clips(&self,overwrite: bool) -> Result<(),Box<dyn Error>> {
        let source_file = self.source_file.as_ref().ok_or("source file must be set before calling extract")?;
        let output_path = self.output_path.as_ref().ok_or("output file must be set before calling extract")?;
        for clip in &self.clips {

            let mut output_file = output_path.clone();
            let output_file_name = self.get_output_filename(clip)?;

            output_file.push(output_file_name);
            crate::ffmpeg::extract_clip(
                clip,
                source_file,
                output_file.as_ref(),
                overwrite
            )?;
        }
        return Ok(());
    }
    
}

fn get_file_ext_from_path(path: &Path) -> Result<String, Box<dyn Error>> {
    return match path.extension() {
        None => Err("need a file extension".into()),
        Some(ext) => match ext.to_str() {
            Some(s) => Ok(String::from(s)),
            None => Err("File extension must be valid UTF-8, no funny business".into())
        }
    };
}
