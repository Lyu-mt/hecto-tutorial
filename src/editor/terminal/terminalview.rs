use std::io::Error;

use super::{Position, Terminal};
use crate::editor::{Coordinate, RenderingError, Size, View};
// clippy::module_name_repetitions: We need to be able to differentiate between the trait View, and the Terminal's instance of a view, hence the prefix.
#[allow(clippy::module_name_repetitions)]
pub struct TerminalView {
    size: Size,
}

/// Represents a portion of the terminal to render onto.
/// Currently, a `TerminalView` needs to satisy the following constraints:
/// - At most as high as the Terminal itself
/// - Exactly as wide as the Terminal itself
/// - Starting at the top left of the Terminal
impl View for TerminalView {
    /// Renders the str into the view, starting at origin
    /// Panics if the str exceeds the length available in this view (debug only)
    fn render_str(&self, str: &str, origin: Coordinate) -> Result<(), RenderingError> {
        debug_assert!(str.len() <= self.size.width.saturating_sub(origin.x));
        Terminal::move_caret_to(origin)?;
        Terminal::clear_until_newline()?;
        Terminal::print(str)?;
        Ok(())
    }
    fn size(&self) -> Size {
        self.size
    }
}

impl TerminalView {
    fn clear(&self) -> Result<(), Error> {
        let mut clear_string = String::new();
        for current_row in 0..self.size.height {
            clear_string.push('~');
            clear_string.push_str(&" ".repeat(self.size().width.saturating_sub(1)));
            if current_row.saturating_add(1) < self.size.height {
                clear_string.push('\r');
                clear_string.push('\n');
            }
        }
        Terminal::move_caret_to(Position::default())?;
        Terminal::print(&clear_string)?;
        Ok(())
    }
    pub fn from(size: Size) -> Result<Self, Error> {
        let terminal_view = Self { size };
        #[cfg(debug_assertions)]
        {
            let terminal_size = Terminal::size().unwrap();
            debug_assert_eq!(terminal_size.width, size.width);
            debug_assert!(terminal_size.height >= size.height);
        }
        terminal_view.clear()?;
        Ok(terminal_view)
    }
}
