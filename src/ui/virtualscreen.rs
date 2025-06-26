use crossterm::style::Color;

use super::ui::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: None,
            bg: None,
        }
    }
}

pub struct VirtualScreen {
    pub size: Size,
    pub cells: Vec<Vec<Cell>>,
}

impl VirtualScreen {
    pub fn new(size: Size) -> Self {
        let cells = vec![vec![Cell::default(); size.width]; size.height];
        Self { size, cells }
    }

    pub fn clear(&mut self) {
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                self.cells[row][col] = Cell::default();
            }
        }
    }

    pub fn set(&mut self, position: Point, ch: char, fg: Option<Color>, bg: Option<Color>) {
        if position.y < self.size.height && position.x < self.size.width {
            self.cells[position.y][position.x] = Cell { ch, fg, bg };
        }
    }
}
