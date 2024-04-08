use std::io::Error;

use crate::editor::Terminal;

pub struct Line;

impl Line {
    pub fn render() -> Result<(), Error> {
        Terminal::print("Hello, World!")?;
        Terminal::print("\r\n")?;
        Ok(())
    }
}
