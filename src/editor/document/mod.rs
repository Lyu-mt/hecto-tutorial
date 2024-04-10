mod line;
use line::Line;
use std::fs;
use std::io::Error;

use super::terminal::{Size, Terminal};
use super::View;

#[derive(Default)]
pub struct Document {
    lines: Vec<Line>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { lines })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn render_into<T: View>(&self, view: &T) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            if let Some(line) = self.lines.get(row) {
                line.render_into(view)?;
            }
        }
        Ok(())
    }
}
