use super::Terminal;
use super::TerminalView;
use crate::editor::{Coordinate,RenderingError, Size, View};

impl View for TerminalView {
    fn render_str(&self, str: &str, _origin: Coordinate) -> Result<(), RenderingError> {
        if let Err(err) = Terminal::print(str) {
            return Err(RenderingError::IO(err));
        }
        Ok(())
    }
    fn size(&self) -> Size {
        Terminal::size().unwrap_or_default()
    }
}
