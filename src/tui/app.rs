use std::{io, time::Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, TableState},
    DefaultTerminal, Frame,
};

use crate::{api::google_sheets::EnergyApi, energy_data::energy_monitor::EnergyMonitor};

use super::widgets::{LeftBottomBlock, RightBottomBlock, UpperBlock};

pub struct App {
    pub data: Vec<EnergyMonitor>,
    pub api: EnergyApi,
    pub exit: bool,
    pub left_bottom_block_state: TableState, // Estado da tabela
}

impl App {
    pub async fn new(api: EnergyApi) -> Result<Self, Box<dyn std::error::Error>> {
        let all_readings = api.get_all_readings().await?;

        let mut data = Vec::with_capacity(all_readings.len());
        for energy_reading in all_readings {
            if let Some(tmp) = EnergyMonitor::new(energy_reading) {
                data.push(tmp);
            }
        }

        let mut left_bottom_block_state = TableState::default();
        if !data.is_empty() {
            left_bottom_block_state.select(Some(0)); // Iniciar na primeira linha
        }

        Ok(App {
            data,
            api,
            exit: false,
            left_bottom_block_state,
        })
    }
    pub async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_time = std::time::Instant::now();

        while !self.exit {
            let _ = terminal.draw(|frame| self.draw(frame));
            self.handle_events()?;

            if last_time.elapsed() >= std::time::Duration::from_secs(60) {
                self.update_data_with_last_reading().await?;
                last_time = Instant::now();
            }
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

    fn draw(&mut self, frame: &mut Frame) {
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);

        let main_block = Block::default()
            .title("Monitoramento de Energia")
            .title_bottom(instructions.centered())
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let main_area = main_block.inner(frame.area());
        frame.render_widget(main_block, frame.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(main_area);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let upper_block = UpperBlock { data: &self.data };
        frame.render_widget(upper_block, chunks[0]);

        // Passando a referência mutável para o LeftBottomBlock
        let left_bottom_block = LeftBottomBlock {
            data: &self.data,
            state: &mut self.left_bottom_block_state, // Passando a referência mutável
        };
        frame.render_widget(left_bottom_block, bottom_chunks[0]);

        let right_bottom_block = RightBottomBlock { data: &self.data };
        frame.render_widget(right_bottom_block, bottom_chunks[1]);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Down => self.move_selection(1), // Avançar na seleção
            KeyCode::Up => self.move_selection(-1),  // Retroceder na seleção
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn move_selection(&mut self, delta: isize) {
        if let Some(selected) = self.left_bottom_block_state.selected() {
            let new_selected = if delta > 0 {
                (selected + delta as usize) % self.data.len()
            } else if selected == 0 {
                self.data.len() - 1
            } else {
                selected - 1
            };
            self.left_bottom_block_state.select(Some(new_selected));
        }
    }
}
