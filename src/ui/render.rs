use super::{ui::Size, virtualscreen::VirtualScreen};
use crossterm::{
    cursor,
    style::{self, ResetColor, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use std::io::{self, Write};

pub struct Renderer {
    pub physical_screen: VirtualScreen,
}

impl Renderer {
    pub fn new(size: Size) -> Self {
        Self {
            physical_screen: VirtualScreen::new(size),
        }
    }

    pub fn render(&mut self, virtual_screen: &mut VirtualScreen) {
        let mut stdout = io::stdout();
        stdout.queue(cursor::Hide).unwrap();

        for y in 0..self.physical_screen.size.height {
            for x in 0..self.physical_screen.size.width {
                let virt_cell = virtual_screen.cells[y][x];
                let phys_cell = self.physical_screen.cells[y][x];

                if virt_cell != phys_cell {
                    stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();

                    if virt_cell.fg != phys_cell.fg {
                        if let Some(fg) = virt_cell.fg {
                            stdout.queue(SetForegroundColor(fg)).unwrap();
                        } else {
                            stdout.queue(ResetColor).unwrap();
                        }
                    }

                    if virt_cell.bg != phys_cell.bg {
                        if let Some(bg) = virt_cell.bg {
                            stdout.queue(SetBackgroundColor(bg)).unwrap();
                        } else {
                            stdout.queue(ResetColor).unwrap();
                        }
                    }

                    stdout.queue(style::Print(virt_cell.ch)).unwrap();
                }
            }
        }

        std::mem::swap(&mut self.physical_screen, virtual_screen);
        stdout.queue(cursor::Show).unwrap();
        stdout.flush().unwrap();
    }
}
