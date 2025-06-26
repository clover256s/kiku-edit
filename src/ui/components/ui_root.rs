use crate::{
    core::document,
    ui::{component::Component, virtualscreen::VirtualScreen},
};

pub struct UIRoot {
    pub childern: Vec<Box<dyn Component>>,
}

impl UIRoot {
    pub fn new() -> Self {
        Self {
            childern: Vec::new(),
        }
    }

    pub fn add_child<C: Component + 'static>(mut self, child: C) -> Self {
        self.childern.push(Box::new(child));
        self
    }
}

impl Component for UIRoot {
    fn draw(&self, screen: &mut VirtualScreen, editor_state: &crate::core::state::EditorState) {
        for child in &self.childern {
            child.draw(screen, editor_state);
        }
    }
}
