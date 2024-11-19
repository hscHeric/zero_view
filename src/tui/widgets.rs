// widgets.rs

use crate::energy_data::energy_monitor::EnergyMonitor;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

// UpperBlock - Gráfico
pub struct UpperBlock<'a> {
    pub data: &'a [EnergyMonitor],
}

impl<'a> Widget for UpperBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Gráfico de Energia ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(ratatui::symbols::border::THICK);

        // Aqui seria o lugar para o gráfico. Vamos simular com um texto por enquanto
        let text = Text::from(vec![Line::from("Simulação de gráfico de energia")]);

        Paragraph::new(text).block(block).render(area, buf);
    }
}

// LeftBottomBlock - Dados de Energia
pub struct LeftBottomBlock<'a> {
    pub data: &'a [EnergyMonitor],
}

impl<'a> Widget for LeftBottomBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Dados de Energia ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(ratatui::symbols::border::THICK);

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

        Paragraph::new(data_text).block(block).render(area, buf);
    }
}

// RightBottomBlock - Picos de Corrente
pub struct RightBottomBlock<'a> {
    pub data: &'a [EnergyMonitor],
}

impl<'a> Widget for RightBottomBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Picos de Corrente ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(ratatui::symbols::border::THICK);

        // Simulando picos de corrente como texto
        let text = Text::from(vec![Line::from("Simulação de picos de corrente")]);

        Paragraph::new(text).block(block).render(area, buf);
    }
}
