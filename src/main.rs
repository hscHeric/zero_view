use std::io::stdout;

use api::google_sheets::EnergyApi;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tui::app::App;

mod api;
mod energy_data;
mod tui;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicialização do terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Cria a API
    let api = EnergyApi::new();

    // Cria o aplicativo
    let mut app = App::new(api).await?;

    // Executa o aplicativo
    let result = app.run(&mut terminal).await;

    // Restaura o terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Retorna o resultado
    result
}
