use crate::energy_data::energy_monitor::EnergyMonitor;
use ratatui::{
    buffer::{Buffer, Cell},
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
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

pub struct LeftBottomBlock<'a> {
    pub data: &'a [EnergyMonitor],
    pub state: &'a mut TableState, // Usar uma referência mutável
}

impl<'a> LeftBottomBlock<'a> {
    // A função `new` também precisa receber uma referência mutável para o estado
    pub fn new(data: &'a [EnergyMonitor], state: &'a mut TableState) -> Self {
        if !data.is_empty() {
            state.select(Some(0)); // Inicia na primeira linha
        }
        Self { data, state }
    }
}

impl<'a> Widget for LeftBottomBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Dados de Energia ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(ratatui::symbols::border::THICK);

        let header_style = Style::default()
            .fg(Color::Yellow)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD);

        let header = Row::new(vec!["Data e Hora", "Potência", "Corrente"])
            .style(header_style)
            .height(1);

        let rows = self.data.iter().map(|monitor| {
            Row::new(vec![
                monitor.timestamp().format("%H:%M:%S").to_string(),
                format!("{:.2} W", monitor.power_watts()),
                format!("{:.2} A", monitor.current_amperes()),
            ])
        });

        let highlight_style = Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ],
        )
        .header(header)
        .block(block)
        .highlight_style(highlight_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ]);

        // Renderizar tabela com estado (para scroll)
        StatefulWidget::render(&table, area, buf, &mut self.state.clone());
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
