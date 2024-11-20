use crate::energy_data::energy_monitor::EnergyMonitor;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, Paragraph, Row, StatefulWidget, Table, TableState, Widget,
    },
};

// UpperBlock - Gráfico
pub struct UpperBlock<'a> {
    pub data: &'a [EnergyMonitor],
}

pub struct UpperBlockState {
    pub offset: usize,
    pub total_groups: usize,
}

impl Default for UpperBlockState {
    fn default() -> Self {
        Self {
            offset: 0,
            total_groups: 0,
        }
    }
}

impl<'a> UpperBlock<'a> {
    fn to_grouped_bar_chart(
        &self,
        area_width: u16,
        state: &mut UpperBlockState,
    ) -> (BarChart<'a>, usize) {
        let barchart = BarChart::default()
            .block(Block::new().title(Line::from("Monitoramento de Energia").centered()))
            .bar_gap(2)
            .bar_width(3)
            .group_gap(4);

        let mut sorted_data = self.data.to_vec();
        sorted_data.sort_by_key(|b| std::cmp::Reverse(b.timestamp()));

        let max_groups = (area_width / (3 * 2 + 1)) as usize;

        state.total_groups = sorted_data.len();

        state.offset = state
            .offset
            .min(sorted_data.len().saturating_sub(max_groups));

        let data: Vec<BarGroup<'_>> = sorted_data
            .iter()
            .skip(state.offset)
            .take(max_groups)
            .map(|monitor| {
                let timestamp_label = monitor.timestamp().format("%H:%M:%S").to_string();
                BarGroup::default()
                    .label(Line::from(timestamp_label))
                    .bars(&[
                        Bar::default()
                            .value(monitor.power_watts().round() as u64)
                            .label(Line::from("W"))
                            .style(Style::default().fg(Color::Green)),
                        Bar::default()
                            .value((monitor.current_amperes() * 100.0).round() as u64)
                            .label(Line::from("A"))
                            .style(Style::default().fg(Color::Blue)),
                    ])
            })
            .collect();

        let chart = data
            .into_iter()
            .fold(barchart, |chart, group| chart.data(group));

        (chart, sorted_data.len())
    }
}

impl<'a> StatefulWidget for UpperBlock<'a> {
    type State = UpperBlockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Line::from(" Gráfico de Energia ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(symbols::border::THICK);

        let (barchart, _) = self.to_grouped_bar_chart(area.width, state);

        let barchart_with_scroll = barchart.block(
            block
                .clone()
                .title(Line::from(" Gráfico de Energia ".bold())),
        );

        barchart_with_scroll.render(area, buf);
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

pub struct RightBottomBlock<'a> {
    pub data: &'a [EnergyMonitor],
}

impl<'a> RightBottomBlock<'a> {
    fn generate_insights(&self) -> Vec<Line<'static>> {
        let current_stats = self.calculate_current_stats();
        let power_stats = self.calculate_power_stats();
        let timestamp_stats = self.calculate_timestamp_stats();

        vec![
            Line::from("📊 Insights de Corrente:").bold(),
            Line::from(format!("• Corrente Máxima: {:.2} A", current_stats.max)),
            Line::from(format!("• Corrente Mínima: {:.2} A", current_stats.min)),
            Line::from(format!("• Corrente Média: {:.2} A", current_stats.average)),
            Line::from("⚡ Insights de Potência:").bold(),
            Line::from(format!("• Potência Máxima: {:.2} W", power_stats.max)),
            Line::from(format!("• Potência Mínima: {:.2} W", power_stats.min)),
            Line::from(format!("• Potência Média: {:.2} W", power_stats.average)),
            Line::from("🕒 Insights de Tempo:").bold(),
            Line::from(format!(
                "• Total de Registros: {}",
                timestamp_stats.total_points
            )),
            Line::from(format!(
                "• Primeiro Registro: {}",
                timestamp_stats.first_timestamp
            )),
            Line::from(format!(
                "• Último Registro: {}",
                timestamp_stats.last_timestamp
            )),
            Line::from(format!(
                "• Duração: {:.2} mins",
                timestamp_stats.duration_minutes
            )),
        ]
    }

    fn calculate_current_stats(&self) -> Statistics {
        let currents: Vec<f64> = self.data.iter().map(|m| m.current_amperes()).collect();
        Statistics {
            max: currents.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            min: currents.iter().cloned().fold(f64::INFINITY, f64::min),
            average: currents.iter().sum::<f64>() / currents.len() as f64,
        }
    }

    fn calculate_power_stats(&self) -> Statistics {
        let powers: Vec<f64> = self.data.iter().map(|m| m.power_watts()).collect();
        Statistics {
            max: powers.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            min: powers.iter().cloned().fold(f64::INFINITY, f64::min),
            average: powers.iter().sum::<f64>() / powers.len() as f64,
        }
    }

    fn calculate_timestamp_stats(&self) -> TimestampStatistics {
        let timestamps: Vec<_> = self.data.iter().map(|m| m.timestamp()).collect();
        TimestampStatistics {
            total_points: timestamps.len(),
            first_timestamp: timestamps.first().unwrap().format("%H:%M:%S").to_string(),
            last_timestamp: timestamps.last().unwrap().format("%H:%M:%S").to_string(),
            duration_minutes: (timestamps.last().unwrap().timestamp()
                - timestamps.first().unwrap().timestamp()) as f64
                / 60.0,
        }
    }
}

impl<'a> Widget for RightBottomBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Informações de Energia ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(ratatui::symbols::border::THICK);

        let insights = self.generate_insights();

        let text = Text::from(insights);
        Paragraph::new(text).block(block).render(area, buf);
    }
}

// Estruturas auxiliares mantidas iguais
struct Statistics {
    max: f64,
    min: f64,
    average: f64,
}

struct TimestampStatistics {
    total_points: usize,
    first_timestamp: String,
    last_timestamp: String,
    duration_minutes: f64,
}
