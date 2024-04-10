use super::Terminal;
use super::TerminalView;
use crate::editor::View;
use std::io::Error;

impl View for TerminalView {
    fn render(&self, str: &str) -> Result<(), Error> {
        Terminal::print(str)
    }
}
