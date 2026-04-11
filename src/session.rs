use crate::clip::Clip;
use std::{error::Error, path::PathBuf};

pub struct Session {
    source_file: Option<PathBuf>,
    output_path: Option<PathBuf>,
    clips: Vec<Clip>,
    session_name: Option<String>,
    concat: bool,
}

impl Session {
    pub fn new(source_file: Option<PathBuf>, output_path: Option<PathBuf>, clips: Vec<Clip>, session_name: Option<String>, concat: bool) -> Session {
        Session { source_file, output_path, clips, session_name, concat }
    }
    pub fn with_defaults() -> Session {
        Session { source_file: None, output_path: None, clips: Vec::new(), session_name: None, concat: false}
    }

    pub fn add_clip(&mut self, clip:Clip) {
        self.clips.push(clip)
    }

    fn get_file_ext(&self) -> Result<String, Box<dyn Error>> {
        let source_file = self.source_file.as_ref().ok_or("source file must be set before calling extract")?;
        return match source_file.extension() {
            None => Err("For now, we need file extension - we aren't letting ffmpeg dynamically discover it".into()),
            Some(ext) => match ext.to_str() {
                Some(s) => Ok(String::from(s)),
                None => Err("File extension must be valid UTF-8, no funny business".into())
            }
        }; 
    }

    fn get_output_filename(&self, clip: &Clip) -> Result<String, Box<dyn Error>> {

        
        // If the session name is set, it overrides individual labels
        let file_label : String = match &self.session_name {
            Some(str) => str.clone(),
            None => String::from(clip.label())
        };

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
