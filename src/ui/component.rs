use crate::{core::state::EditorState, ui::virtualscreen::VirtualScreen};

pub trait Component {
    fn draw(&self, screen: &mut VirtualScreen, editor_state: &EditorState);
}
