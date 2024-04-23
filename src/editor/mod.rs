use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;
use std::{env, panic};

mod terminal;
use terminal::{Position, Terminal, TerminalView};

mod document;
use document::Document;

mod prelude;
pub use prelude::*;

const NAME: &str = env!("CARGO_PKG_NAME");

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    document: Document,
}

type Location = Coordinate;

impl Editor {
    pub fn run(&mut self) {
        let current_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize().unwrap();
        self.load_doc_from_args();
        self.repl().unwrap();
    }
    fn load_doc_from_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            if let Ok(loaded_doc) = Document::open(file_name) {
                self.document = loaded_doc;
            }
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            let event = read()?;
            self.evaluate_event(&event);
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) {
        let Size { height, .. } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Up => self.document.move_point(Direction::Up(1)),
            KeyCode::Down => self.document.move_point(Direction::Down(1)),
            KeyCode::Left => self.document.move_point(Direction::Left(1)),
            KeyCode::Right => self.document.move_point(Direction::Right(1)),
            KeyCode::PageUp => self.document.move_point(Direction::Up(height)),
            KeyCode::PageDown => self.document.move_point(Direction::Down(height)),
            KeyCode::Home => self.document.move_point(Direction::StartOfLine),
            KeyCode::End => self.document.move_point(Direction::EndOfLine),
            _ => (),
        }
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code);
                }
                _ => (),
            }
        }
    }
    /// Refreshs the screen by hiding the caret, moving it to the top left and drawing the new screen.
    /// Errors regarding showing/hiding the caret are silently ignored as we'd expect the caret
    /// to return at the latest upon the next refresh.
    /// Errors regarding to drawing and caret placement would lead to undefined behaviour and are therefore propagated up.
    fn refresh_screen(&self) -> Result<(), Error> {
        let _ = Terminal::hide_caret();
        Terminal::move_caret_to(Position::default())?;

        self.draw_rows()?;
        let point = self.document.point_location();
        Terminal::move_caret_to(Position {
            x: point.x,
            y: point.y,
        })?;
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
        Ok(())
    }
    fn render_welcome_message_into<T: View>(view: &T) -> Result<(), RenderingError> {
        let Size { height, width } = view.size();
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if height < 1 || width < len {
            return Ok(());
        }
        // sets the vertical position of the welcome message to be roughly 1/3 of the screen.
        // sets the horizontal position of the welcome message roughly to the middle.
        // we allow integer division here since we do not care if it is centered _exactly_ right.
        #[allow(clippy::integer_division)]
        let origin = Position {
            y: height / 3,
            x: ((width.saturating_sub(len)) / 2).saturating_sub(1),
        };

        view.render_str(&welcome_message, origin)?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), Error> {
        let size = Terminal::size().unwrap_or_default();
        let terminal_view = TerminalView::from(size)?;
        if self.document.is_empty() {
            Self::render_welcome_message_into(&terminal_view)?;
        } else {
            self.document.render_into(&terminal_view)?;
        }
        Ok(())
    }
}
impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
