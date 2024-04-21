use std::io::Error;

use super::{Position, Terminal};
use crate::editor::{Coordinate, RenderingError, Size, View};
// clippy::module_name_repetitions: We need to be able to differentiate between the trait View, and the Terminal's instance of a view, hence the prefix.
#[allow(clippy::module_name_repetitions)]
pub struct TerminalView {
    size: Size,
}

impl View for TerminalView {
    fn render_str(&self, str: &str, origin: Coordinate) -> Result<(), RenderingError> {
        Terminal::move_caret_to(origin)?;
        Terminal::clear_line()?;
        Terminal::print(str)?;
        Ok(())
    }
    fn size(&self) -> Size {
        self.size
    }
}

impl TerminalView {
    fn clear(&self) -> Result<(), Error> {
        Terminal::move_caret_to(Position {
            x: self.size.width,
            y: self.size.height,
        })?;
        Terminal::clear_from_cursor_up()?;
        Ok(())
    }
    pub fn from(size: Size) -> Result<Self, Error> {
        let terminal_view = Self { size };
        terminal_view.clear()?;
        Ok(terminal_view)
    }
}
