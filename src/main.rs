use std::error::Error;

use crossterm::event::{Event, EventStream};
use futures::{FutureExt, StreamExt};
use ratatui::DefaultTerminal;
use tokio::time;

use tui_components_demo::components::{app, Component};

type Result = std::result::Result<(), Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    let result = run_app(terminal, 60.0).await;
    ratatui::restore();
    result
}

async fn run_app(mut terminal: DefaultTerminal, framerate: f64) -> Result {
    let mut app_component = app::App::new();

    let mut reader = EventStream::new();

    let mut tick_interval = time::interval(time::Duration::from_secs_f64(1.0 / framerate));

    loop {
        // Wait for next tick or key event
        tokio::select! {
            _ = tick_interval.tick() => {
                terminal.draw(|frame| app_component.render(app::AppProps{}, frame, frame.area()))?;
            }

            event = reader.next().fuse() => match event {
                Some(Ok(Event::Key(key_event))) => {
                    if let Some(app::AppActions::Quit) = app_component.handle_key(app::AppProps{}, key_event.code) {
                        return Ok(())
                    }
                }
                _ => {},
            }
        }
    }
}
