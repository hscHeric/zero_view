use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};

use crate::{api::google_sheets::EnergyApi, energy_data::energy_monitor::EnergyMonitor};

use super::widgets::{LeftBottomBlock, RightBottomBlock, UpperBlock};

pub struct App {
    pub data: Vec<EnergyMonitor>,
    pub api: EnergyApi,
    exit: bool,
}

impl App {
    pub async fn new(api: EnergyApi) -> Result<Self, Box<dyn std::error::Error>> {
        let all_readings = api.get_all_readings().await?;

        // Processa as leituras e converte para EnergyMonitor
        let mut data = Vec::with_capacity(all_readings.len());

        for energy_reading in all_readings {
            if let Some(tmp) = EnergyMonitor::new(energy_reading) {
                data.push(tmp);
            }
        }

        Ok(App {
            data,
            api,
            exit: false,
        })
    }

    pub async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            let _ = terminal.draw(|frame| self.draw(frame));
            self.handle_events()?;
            self.update_data_with_last_reading().await?;
        }
        Ok(())
    }
    async fn update_data_with_last_reading(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let last_energy_reading = self.api.get_last_reading().await?;
        if let Some(monitor) = EnergyMonitor::new(last_energy_reading) {
            if self.data.last() != Some(&monitor) {
                self.data.push(monitor);
            }
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);

        // Define o bloco principal que encapsula tudo
        let main_block = Block::default()
            .title("Monitoramento de Energia")
            .title_bottom(instructions.centered())
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        // Calcula a área interna do bloco principal
        let main_area = main_block.inner(frame.area());

        // Renderiza o bloco principal
        frame.render_widget(main_block, frame.area());

        // Divide o layout dentro do bloco principal
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(30), // Parte superior ocupa 30% da altura
                Constraint::Percentage(70), // Parte inferior ocupa 70% da altura
            ])
            .split(main_area);

        // Divide a área inferior em dois blocos horizontais
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Inferior esquerdo ocupa 50% da largura
                Constraint::Percentage(50), // Inferior direito ocupa 50% da largura
            ])
            .split(chunks[1]);

        // Renderiza o bloco superior (gráfico)
        let upper_block = UpperBlock { data: &self.data };
        frame.render_widget(upper_block, chunks[0]);

        // Renderiza o bloco inferior esquerdo (dados)
        let left_bottom_block = LeftBottomBlock { data: &self.data };
        frame.render_widget(left_bottom_block, bottom_chunks[0]);

        // Renderiza o bloco inferior direito (picos de corrente)
        let right_bottom_block = RightBottomBlock { data: &self.data };
        frame.render_widget(right_bottom_block, bottom_chunks[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char('q') = key_event.code {
            self.exit = true
        }
    }
}
