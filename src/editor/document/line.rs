use super::View;
use crate::editor::{Coordinate, RenderingError};

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(str: &str) -> Self {
        Self {
            string: String::from(str),
        }
    }
    pub fn render_into<T: View>(&self, view: &T, origin: Coordinate) -> Result<(), RenderingError> {
        view.render_str(&self.string, origin)?;
        Ok(())
    }
}
