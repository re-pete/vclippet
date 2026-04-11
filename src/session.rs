use crate::clip::Clip;
use std::{error::Error, path::PathBuf};

pub struct Session {
    source_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
    clips: Vec<Clip>,
    session_name: Option<String>,
    concat: bool,
}

impl Session {
    pub fn new(source_file: Option<PathBuf>, output_file: Option<PathBuf>, clips: Vec<Clip>, session_name: Option<String>, concat: bool) -> Session {
        Session { source_file, output_file, clips, session_name, concat }
    }
    pub fn with_defaults() -> Session {
        Session { source_file: None, output_file: None, clips: Vec::new(), session_name: None, concat: false}
    }

    pub fn add_clip(&mut self, clip:Clip) {
        self.clips.push(clip)
    }

    fn get_output_filename(&self, clip: &Clip, file_ext: &str) -> Result<String, Box<dyn Error>> {

        
        // If the session name is set, it overrides individual labels
        let file_label : String = match &self.session_name {
            Some(str) => str.clone(),
            None => String::from(clip.label())
        };

        let file_ext : &str = match self.source_file.as_ref().expect("source file must be set before calling extract").extension() {
            None => return Err("For now, we need file extension - we aren't letting ffmpeg dynamically discover it".into()),
            Some(ext) => match ext.to_str() {
                Some(s) => s,
                None => return Err("File extension must be valid UTF-8, no funny business".into())
            }
        };
        // If we are concatenating a bunch of clips, we don't worry about timestamps
        let clip_str = if self.concat {
            String::from("")
        } else {
            format!("_{}-{}", clip.start, clip.end)
        };
        return Ok(format!("{}{}.{}",file_label,clip_str,file_ext));
    }
    
    pub fn extract_clips(&self) -> Result<(),Box<dyn Error>> {
        for clip in &self.clips {
            crate::ffmpeg::extract_clip(
                clip,
                &self.source_file.as_ref().expect("source file must be set before calling extract"),
                &self.output_file.as_ref().expect("output file must be set before calling extract"),
                self.concat
            )?;
        }
        return Ok(());
    }
    
}
