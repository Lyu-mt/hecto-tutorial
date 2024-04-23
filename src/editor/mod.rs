use core::cmp::min;
use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;
use std::{env, panic};
mod terminal;
use terminal::{Position, Terminal, TerminalView};

use self::document::Document;
mod document;

const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Copy, Clone, Default)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum RenderingError {
    IO(Error),
}

impl From<Error> for RenderingError {
    fn from(error: Error) -> Self {
        Self::IO(error)
    }
}
impl From<RenderingError> for Error {
    fn from(rendering_error: RenderingError) -> Self {
        match rendering_error {
            RenderingError::IO(io_error) => io_error,
        }
    }
}

pub trait View {
    fn size(&self) -> Size;
    fn render_str(&self, str: &str, origin: Coordinate) -> Result<(), RenderingError>;
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
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
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
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
        Terminal::move_caret_to(Position {
            x: self.location.x,
            y: self.location.y,
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
