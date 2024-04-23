use std::io::Error;
#[derive(Copy, Clone)]
pub enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
    StartOfLine,
    EndOfLine,
}
#[derive(Copy, Clone, Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

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
