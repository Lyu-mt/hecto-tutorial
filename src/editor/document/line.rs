use super::View;
use std::io::Error;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(str: &str) -> Self {
        Self {
            string: String::from(str),
        }
    }
    pub fn render_into<T: View>(&self, view: &T) -> Result<(), Error> {
        view.render(&self.string)?;
        view.render("\r\n")?;
        Ok(())
    }
}
