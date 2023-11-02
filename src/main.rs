mod player_state;
mod cell_state;
mod border_policy;
mod app_layout;
mod double_buffer_grid;
mod app;
mod app_event;
mod handler;
mod tui;
mod ui;

use app_event::EventHandler;
use handler::handle_key_events;
use ratatui::prelude::*;
use tui::Tui;
use std::io::stdout;

use app::App;

fn main() -> anyhow::Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?.clone();
    let events = EventHandler::new(100);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let mut app = App::new(size);
    app.randomize_cells();

    while !app.should_quit {
        match tui.events.next()? {
            app_event::Event::Tick => app.tick(),
            app_event::Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            app_event::Event::Mouse(_) => {},
            app_event::Event::Resize(_, _) => {},
            app_event::Event::None => {},
        }

        // app.logic_update()?;
        tui.draw(&mut app)?;
    }

    tui.exit()?;

    Ok(())
}
