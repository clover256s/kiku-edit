use crate::{
    core::document::Document,
    ui::{
        component::Component,
        ui::{Point, Size},
        virtualscreen::VirtualScreen,
    },
};
use unicode_width::UnicodeWidthChar;

pub struct TextView {
    pub position: Point,
    pub scroll: Point,
    pub size: Size,
}

impl TextView {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position,
            scroll: Point::default(),
            size,
        }
    }
}

impl Component for TextView {
    fn draw(&self, screen: &mut VirtualScreen, editor_state: &crate::core::state::EditorState) {
        let document = &editor_state.document;
        for row in self.position.y..self.size.height {
            let y = self.scroll.y + row.saturating_sub(self.position.y);
            if y < document.lines.len() {
                let line = &document.lines[y];
                let mut x = 0;
                let mut col = 0;

                while col < self.size.width {
                    let ch = line.text().chars().nth(x).unwrap_or(' ');
                    let cw = ch.width().unwrap_or(1);

                    screen.set(Point::new(col, row), ch, None, None);
                    if cw == 2 && col < self.size.width {
                        screen.set(Point::new(col, row), '\0', None, None);
                    }

                    x += 1;
                    col += cw;
                }
            } else {
                screen.set(Point::new(0, row), '~', None, None);
                for x in 1..self.size.width {
                    screen.set(Point::new(x, row), ' ', None, None);
                }
            }
        }
    }
}
