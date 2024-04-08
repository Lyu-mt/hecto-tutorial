mod line;
use std::io::Error;

use line::Line;

pub struct Document;

impl Document {
    pub fn render() -> Result<(), Error> {
        Line::render()?;
        Ok(())
    }
}
