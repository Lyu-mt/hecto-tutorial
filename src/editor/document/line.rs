use crate::editor::{Coordinate, RenderingError};
use super::View;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(str: &str) -> Self {
        Self {
            string: String::from(str),
        }
    }
    pub fn render_into<T: View>(&self, view: &T) -> Result<(), RenderingError> {
        view.render_str(&self.string, Coordinate::default())?;
        view.render_str("\r\n", Coordinate::default())?;
        Ok(())
    }
}
