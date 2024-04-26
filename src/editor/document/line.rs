use core::cmp::min;

use crate::editor::{Coordinate, Location, RenderingError, View};

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
    pub fn len(&self) -> usize {
        self.string.len()
    }
    pub fn render_into<T: View>(
        &self,
        view: &T,
        origin: Coordinate,
        scroll_offset: Location,
    ) -> Result<(), RenderingError> {
        let width = view.size().width.saturating_sub(origin.x);
        let end = min(scroll_offset.x.saturating_add(width), self.string.len());
        let start = scroll_offset.x.saturating_add(origin.x);
        let substring = self.string.get(start..end).unwrap_or("");
        view.render_str(substring, origin)?;
        Ok(())
    }
}
