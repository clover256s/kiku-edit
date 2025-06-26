use std::io;

use crossterm::{
    style::{Color, SetAttribute, SetBackgroundColor},
    QueueableCommand,
};

use crate::{
    core::document,
    ui::{component::Component, ui::Point},
};

pub struct StatusBar;

impl Component for StatusBar {
    fn draw(
        &self,
        screen: &mut crate::ui::virtualscreen::VirtualScreen,
        editor_state: &crate::core::state::EditorState,
    ) {
        let document = &editor_state.document;
        let file_name = editor_state
            .document
            .file_path
            .as_ref()
            .map(|path| {
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            })
            .unwrap_or_else(|| "untitled".to_string());

        let message = format!(
            " {} ({}, {}) | {} | {}",
            file_name,
            editor_state.cursor.x,
            editor_state.cursor.y,
            document.encoding,
            document.language
        );

        for x in 0..screen.size.width {
            let ch = message.chars().nth(x).unwrap_or(' ');
            screen.set(
                Point::new(x, screen.size.height - 1),
                ch,
                Some(Color::Black),
                Some(Color::White),
            );
        }
    }
}
