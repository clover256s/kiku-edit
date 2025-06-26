use crossterm::{
    terminal::{self, Clear},
    QueueableCommand,
};
use kiku::{core::state::EditorState, ui::ui::UI};
use std::{io, thread::sleep, time::Duration};

fn main() {
    let _ = terminal::enable_raw_mode().unwrap();
    io::stdout().queue(Clear(terminal::ClearType::All)).unwrap();
    let state = EditorState::new("./target/debug/kiku".into());
    let mut ui = UI::new();
    ui.redraw(&state);

    sleep(Duration::from_secs(5));
    let _ = terminal::disable_raw_mode().unwrap();
}
