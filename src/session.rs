use crate::clip::Clip;
use std::path::PathBuf;

pub struct Session {
    source_file: Option<PathBuf>,
    clips: Vec<Clip>,
    session_name: Option<String>,
    concat: bool,
}

impl Session {
    pub fn new(source_file: Option<PathBuf>, clips: Vec<Clip>, session_name: Option<String>, concat: bool) -> Session {
        Session { source_file, clips, session_name, concat }
    }
    pub fn with_defaults() -> Session {
        Session { source_file: None, clips: Vec::new(), session_name: None, concat: false}
    }

    pub fn add_clip(&mut self, clip:Clip) {
        self.clips.push(clip)
    }
}
