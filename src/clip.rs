pub const DEFAULT_CLIP_LABEL: &str = "output";

pub struct Clip {
  pub start: u32,
  pub end: u32,
  pub label: Option<String>,
}

impl Clip {
    pub fn new(start: u32, end: u32, name: Option<String>) -> Result<Clip,String> {
        if end <= start {
            return Err(String::from("end must be greater than start"));
        }
        Ok(  Clip { start, end, label: name })
    }

    pub fn label(&self) -> &str {
        return self.label.as_deref().unwrap_or(DEFAULT_CLIP_LABEL);
    }
}


