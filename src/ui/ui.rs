use super::{
    component::Component,
    components::{status_bar::StatusBar, text_view::TextView, ui_root::UIRoot},
    render::Renderer,
    virtualscreen::VirtualScreen,
};
use crate::core::{document::Document, state::EditorState};
use crossterm::terminal;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct UI {
    pub screen: VirtualScreen,
    pub renderer: Renderer,
    pub root: UIRoot,
}

impl UI {
    pub fn new() -> Self {
        let (width, height) = terminal::size()
            .map(|(w, h)| (w as usize, h as usize))
            .unwrap();
        let root = UIRoot::new()
            .add_child(TextView::new(
                Point::new(0, 0),
                Size::new(width, height.saturating_sub(1)),
            ))
            .add_child(StatusBar);

        Self {
            screen: VirtualScreen::new(Size::new(width, height)),
            renderer: Renderer::new(Size::new(width, height)),
            root,
        }
    }

    pub fn redraw(&mut self, editor_state: &EditorState) {
        self.root.draw(&mut self.screen, editor_state);
        self.renderer.render(&mut self.screen);
    }
}
