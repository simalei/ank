use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::App;

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event {
        KeyEvent { code: KeyCode::Char('q'), .. } => { app.quit = true; }
        _ => {}
    }
}

pub fn handle_events(app: &mut App, event: Event) {
    match event {
        Event::Key(key_event) => { handle_key_event(app, key_event); }
        _ => {}
    }
}
