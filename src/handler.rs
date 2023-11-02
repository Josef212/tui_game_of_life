use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::App;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> anyhow::Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('r') => {
            if key_event.kind == KeyEventKind::Release {
                app.randomize_cells();
                app.cycle_count = 0;
            }
        }
        KeyCode::Char('p') => {
            if key_event.kind == KeyEventKind::Release {
                app.player_state.switch();
            }
        }
        KeyCode::Char('b') => {
            if key_event.kind == KeyEventKind::Release {
                app.border_policy.switch();
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
