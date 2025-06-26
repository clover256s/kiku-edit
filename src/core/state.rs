use super::cursor::Cursor;
use crate::{core::document::Document, utils::visual_to_logical};

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
}

pub struct EditorState {
    pub is_dirty: bool,
    pub cursor: Cursor,
    pub mode: EditorMode,
    pub document: Document,
}

impl EditorState {
    pub fn new(path: String) -> Self {
        let document = Document::new().load(path);
        Self {
            is_dirty: false,
            cursor: Cursor::default(),
            mode: EditorMode::Normal,
            document,
        }
    }

    fn insert(&mut self, ch: char) {
        if self.mode == EditorMode::Insert {
            self.document.insert(self.cursor.y, self.cursor.x, ch);
            self.cursor.move_right(&self.document);
            self.is_dirty = true;
        }
    }

    fn remvoe(&mut self) {
        // Normal模式下删除是 d or x, Insert 则是 Backspace
        if self.mode == EditorMode::Insert {
            if self.cursor.x > 0 {
                self.document.remove(self.cursor.y, self.cursor.x);
                self.cursor.move_left(&self.document);
            } else if self.cursor.y > 0 {
                let cur_line = self.document.remove_line(self.cursor.y);
                let prev_line = &mut self
                    .document
                    .lines
                    .get_mut(self.cursor.y.saturating_sub(1))
                    .unwrap();
                let prev_len = prev_line.len_chars();

                prev_line.mtext().push_str(cur_line.text());
                self.cursor.y -= 1;
                self.cursor.x = prev_len;
                self.cursor.visual_x = visual_to_logical(self.cursor.x, prev_line.text());
            }

            self.is_dirty = true;
        } else if self.mode == EditorMode::Normal {
            self.document.remove(self.cursor.y, self.cursor.x);
            self.is_dirty = true;
        }
    }
}
