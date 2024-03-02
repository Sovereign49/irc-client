use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Char(x) => {
            app.user_msg.push(x);
        }
        KeyCode::Backspace => {
            app.user_msg.pop();
        }
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Add message to the messages, then clears message prompt on `ENTER`
        KeyCode::Enter => {
            // TODO: add user's name in place of you
            app.messages.push(format!("you: {}", app.user_msg.clone()));
            app.user_msg.clear();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub fn handle_message_event() -> AppResult<()> {
    Ok(())
}
