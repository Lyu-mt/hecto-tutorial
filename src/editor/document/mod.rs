use crate::editor::{Coordinate, Direction, Location, RenderingError, Size, View};
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
    pub fn move_point(&mut self, direction: Direction) {
        match direction {
            Direction::Up(step) => {
                self.point_location.y = self.point_location.y.saturating_sub(step);
            }
            Direction::Down(step) => {
                self.point_location.y = self.point_location.y.saturating_add(step);
            }
            Direction::Left(step) => {
                self.point_location.x = self.point_location.x.saturating_sub(step);
            }
            Direction::Right(step) => {
                self.point_location.x = self.point_location.x.saturating_add(step);
            }
            Direction::StartOfLine => {
                self.point_location.x = 0;
            }
            Direction::EndOfLine => {
                self.point_location.x = self.point_location.x.saturating_add(5);
            }
        };
    }
    pub const fn point_location(&self) -> Location {
        self.point_location
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn render_into<T: View>(&self, view: &T) -> Result<(), RenderingError> {
        let Size { height, .. } = view.size();
        for row in 0..height {
            if let Some(line) = self.lines.get(row) {
                line.render_into(view, Coordinate { x: 0, y: row })?;
            }
        }
        Ok(())
    }
}
