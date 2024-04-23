use core::cmp::min;

use super::View;
use crate::editor::{Coordinate, RenderingError};

pub struct Line {
    string: String,
}

/// Represents a line in the text.
/// Panics:
/// * Debug only: If the passed string contains newlines.
impl Line {
    pub fn from(str: &str) -> Self {
        let string = String::from(str);

        #[cfg(debug_assertions)]
        {
            if string.is_empty() {
                assert_eq!(string.lines().count(), 0);
            } else {
                assert_eq!(string.lines().count(), 1);
            }
        }
        Self { string }
    }
    pub fn render_into<T: View>(&self, view: &T, origin: Coordinate) -> Result<(), RenderingError> {
        let width = view.size().width.saturating_sub(origin.x);
        let substring = self
            .string
            .get(0..min(width, self.string.len()))
            .unwrap_or("");
        view.render_str(substring, origin)?;
        Ok(())
    }
}
