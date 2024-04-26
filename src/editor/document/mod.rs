use crate::editor::{Coordinate, Direction, Location, RenderingError, Size, View};
use std::cmp::min;
use std::fs;
use std::io::Error;

mod line;
use line::Line;

#[derive(Default)]
pub struct Document {
    lines: Vec<Line>,
    point_location: Location,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self {
            lines,
            point_location: Location::default(),
        })
    }
    fn width_at(&self, at: usize) -> usize {
        self.lines.get(at).map_or(0, Line::len)
    }
    /// Moves the point around in the text according to the specified direction.
    /// Specific behaviors:
    /// - Allows placing the point to the right of the last character or one line below the last line.
    /// - Moving left at the start of a line moves the point up and to the end of the previous line.
    /// - Moving right at the end of a line moves the point down and to the start of the next line.
    pub fn move_point(&mut self, direction: Direction) {
        let height = self.lines.len();
        match direction {
            Direction::Up(step) => {
                self.point_location.y = self.point_location.y.saturating_sub(step);
                let width = self.width_at(self.point_location.y);
                self.point_location.x = min(width, self.point_location.x);
            }
            Direction::Down(step) => {
                let newy = self.point_location.y.saturating_add(step);
                self.point_location.y = min(newy, height);
                let width = self.width_at(self.point_location.y);
                self.point_location.x = min(width, self.point_location.x);
            }
            Direction::Left => {
                if self.point_location.x > 0 {
                    self.point_location.x = self.point_location.x.saturating_sub(1);
                } else if self.point_location.y > 0 {
                    // We use recursion here to express that a step to the left at the begining of a line
                    // is the same as moving up one line and moving to the end of the line.
                    self.move_point(Direction::Up(1));
                    self.move_point(Direction::EndOfLine);
                }
            }
            Direction::Right => {
                let width = self.width_at(self.point_location.y);
                if self.point_location.x < width {
                    self.point_location.x = self.point_location.x.saturating_add(1);
                } else if self.point_location.y < height {
                    // We use recursion here to express that a step to the right at the end of a line
                    // is the same as moving down one line and moving to the  start of the line.
                    self.move_point(Direction::Down(1));
                    self.move_point(Direction::StartOfLine);
                }
            }
            Direction::StartOfLine => {
                self.point_location.x = 0;
            }
            Direction::EndOfLine => {
                let width = self.width_at(self.point_location.y);
                self.point_location.x = width;
            }
        };
    }
    pub const fn point_location(&self) -> Location {
        self.point_location
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn render_into<T: View>(
        &self,
        view: &T,
        scroll_offset: Location,
    ) -> Result<(), RenderingError> {
        let Size { height, .. } = view.size();
        for row in 0..height {
            if let Some(line) = self.lines.get(scroll_offset.y.saturating_add(row)) {
                line.render_into(view, Coordinate { x: 0, y: row }, scroll_offset)?;
            } else {
                view.render_str("~", Coordinate { x: 0, y: row })?;
            }
        }
        Ok(())
    }
}
