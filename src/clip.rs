pub struct Clip {
  pub start: u32,
  pub end: u32,
  pub name: Option<String>,
}

impl Clip {
    pub fn new(start: u32, end: u32, name: Option<String>) -> Result<Clip,String> {
        if end <= start {
            return Err(String::from("end must be greater than start"));
        }
        Ok(  Clip { start, end, name })
    }
}


