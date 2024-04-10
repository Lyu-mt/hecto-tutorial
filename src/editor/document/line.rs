use std::io::Error;

use crate::editor::Terminal;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(str: &str) -> Self {
        Self {
            string: String::from(str),
        }
    }
    pub fn render(&self) -> Result<(), Error> {
        Terminal::print(&self.string)?;
        Terminal::print("\r\n")?;
        Ok(())
    }
}
