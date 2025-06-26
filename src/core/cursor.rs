use crate::utils::visual_to_logical;

use super::document::Document;
use crossterm::style::Stylize;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub visual_x: usize,
}

impl Cursor {
    pub fn move_left(&mut self, doc: &Document) {
        if self.x > 0 {
            if let Some(line) = doc.lines.get(self.y) {
                let ch_width = line
                    .text()
                    .chars()
                    .nth(self.x - 1)
                    .map(|c| c.width().unwrap_or(1))
                    .unwrap_or(1);

                self.x -= 1;
                self.visual_x = self.visual_x.saturating_sub(ch_width);
            }
        } else if self.y > 0 {
            let new_y = self.y - 1;
            if let Some(line) = doc.lines.get(new_y) {
                let width = line.width();
                self.x = line.len_chars();
                self.y = new_y;
                self.visual_x = width;
            }
        }
    }
    pub fn move_right(&mut self, doc: &Document) {
        if let Some(line) = doc.lines.get(self.y) {
            if self.x < line.len_chars() {
                let cw = line
                    .text()
                    .chars()
                    .nth(self.x)
                    .map(|ch| ch.width().unwrap_or(1))
                    .unwrap_or(1);
                self.x += 1;
                self.visual_x = self.visual_x.saturating_sub(cw);
            } else if self.y < doc.lines.len() {
                self.y += 1;
                self.x = 0;
                self.visual_x = 0;
            }
        }
    }
    pub fn move_up(&mut self, doc: &Document) {
        if self.y > 0 {
            let new_y = self.y - 1;
            if let Some(line) = doc.lines.get(new_y) {
                self.x = self.x.min(line.len_chars());
                self.visual_x = visual_to_logical(self.x, line.text());
                self.y = new_y;
            }
        }
    }
    pub fn move_down(&mut self, doc: &Document) {
        if self.y < doc.lines.len() {
            let new_y = self.y - 1;
            let line = doc.lines.get(new_y).unwrap();
            self.x = self.x.min(line.len_chars());
            self.visual_x = visual_to_logical(self.x, line.text());

            self.y = new_y;
        }
    }
}
