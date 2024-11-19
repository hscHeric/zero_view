use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{api::google_sheets::EnergyApi, energy_data::energy_monitor::EnergyMonitor};

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
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // Layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(30), // Bloco superior ocupa 30% da altura
                Constraint::Percentage(70), // Bloco inferior ocupa 70%
            ])
            .split(frame.area());

        // Layout para os dois blocos inferiores
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Cada bloco ocupa 50% da largura
                Constraint::Percentage(50),
            ])
            .split(chunks[1]);

        let upper_block = Block::default()
            .title("Bloco Superior")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(upper_block, chunks[0]);

        // Bloco inferior esquerdo: Renderiza o widget &App
        frame.render_widget(self, bottom_chunks[0]);

        // Bloco inferior direito
        let right_block = Block::default()
            .title("Bloco Inferior Direito")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(right_block, bottom_chunks[1]);

        //frame.render_widget(self, frame.area());
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Show data ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let data_text = Text::from(
            self.data
                .iter()
                .map(|monitor| {
                    Line::from(vec![
                        Span::styled(
                            format!("{} ", monitor.timestamp().format("%H:%M:%S")),
                            Style::default().fg(Color::White),
                        ),
                        Span::styled(
                            format!("Power: {:.2}W ", monitor.power_watts()),
                            Style::default().fg(Color::Blue),
                        ),
                        Span::styled(
                            format!("Current: {:.2}A", monitor.current_amperes()),
                            Style::default().fg(Color::Yellow),
                        ),
                    ])
                })
                .collect::<Vec<Line>>(),
        );

        Paragraph::new(data_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
