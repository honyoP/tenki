mod app;
mod editor;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io, path::PathBuf};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get the notes directory from args or use current directory
    let notes_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    let mut app = App::new(notes_dir)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the application
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                (KeyCode::Char('e'), _) => {
                    if let Some(path) = app.selected_file() {
                        // Suspend TUI and open editor
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;

                        editor::open_in_editor(&path)?;

                        // Resume TUI
                        enable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            EnterAlternateScreen,
                            EnableMouseCapture
                        )?;
                        terminal.clear()?;

                        // Refresh the graph after editing
                        app.refresh()?;
                    }
                }
                (KeyCode::Tab, _) => {
                    app.next_pane();
                }
                (KeyCode::BackTab, _) => {
                    app.prev_pane();
                }
                (KeyCode::Up | KeyCode::Char('k'), _) => {
                    app.move_up();
                }
                (KeyCode::Down | KeyCode::Char('j'), _) => {
                    app.move_down();
                }
                (KeyCode::Enter, _) => {
                    app.select();
                }
                (KeyCode::Char('r'), _) => {
                    app.refresh()?;
                }
                _ => {}
            }
        }
    }
}
