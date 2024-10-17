use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

use super::state::State;

pub fn render(state: &mut State, frame: &mut Frame) {
    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
        .spacing(1)
        .split(frame.area());

    let title_line = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(screen[0]);

    let tabs = Tabs::new(Tab::get_headers().to_vec())
        .fg(ThemeColor::Text)
        .highlight_style(Style::default().fg(ThemeColor::Gold.into()))
        .select(state.tab as usize)
        .divider("|")
        .padding(" ", " ");

    frame.render_widget(tabs, title_line[0]);
    let label = Paragraph::new(Line::from(vec![
        Span::from(" parqour ")
            .bg(ThemeColor::Foam)
            .fg(ThemeColor::HighlightLow),
        Span::from(format!(" {} ", state.viewer.filename()))
            .bg(ThemeColor::Pine)
            .fg(ThemeColor::Text),
    ]))
    .right_aligned();
    frame.render_widget(label, title_line[1]);

    match state.tab {
        Tab::Data => {
            frame.render_widget(Paragraph::new("Data"), screen[1]);
        }
        Tab::Metadata => render_metadata(state, frame, screen[1]),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tab {
    Data = 0,
    Metadata = 1,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Data
    }
}

impl Tab {
    const fn get_headers() -> &'static [&'static str] {
        &["Data", "Metadata"]
    }
}

impl From<usize> for Tab {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::Data,
            1 => Self::Metadata,
            _ => Self::default(),
        }
    }
}

pub const N_TABS: usize = Tab::get_headers().len() - 1;

pub fn render_metadata(state: &mut State, frame: &mut Frame, rect: Rect) {
    let parquet_metadata = state.viewer.parquet_metadata();
    let file_metadata = parquet_metadata.file_metadata();
    let parquet_schema = file_metadata.schema_descr();

    let metadata_height = 6 + file_metadata
        .key_value_metadata()
        .map_or(0, |kv| kv.len() + 1) as u16;
    let layout = Layout::vertical([
        Constraint::Min(metadata_height),
        Constraint::Percentage(100),
    ])
    .split(rect);
    let file_metadata_block = Block::bordered()
        .title("File metadata".bold())
        .fg(ThemeColor::Text);

    let parquet_schema_block = Block::bordered()
        .title("Schema".bold())
        .fg(ThemeColor::Text);

    let mut file_metadata_lines = vec![
        Line::from(vec![
            Span::styled("Version: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(file_metadata.version().to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Number of rows: ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(file_metadata.num_rows().to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Number of columns: ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(parquet_schema.num_columns().to_string()),
        ]),
        Line::from(vec![
            Span::styled("Created by: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(file_metadata.created_by().unwrap_or("")),
        ]),
    ];
    if let Some(kv_data) = file_metadata.key_value_metadata() {
        if kv_data.is_empty() {
            return;
        }
        file_metadata_lines.push(Line::from(vec![Span::styled(
            "Other",
            Style::default().fg(ThemeColor::Iris.into()).bold(),
        )]));
        for kv in kv_data.iter() {
            let l = Line::from(vec![
                Span::styled(
                    format!("{}: ", kv.key),
                    Style::default().fg(ThemeColor::Love.into()),
                ),
                Span::raw(kv.value.as_deref().unwrap_or("")),
            ]);
            file_metadata_lines.push(l);
        }
    }
    let file_meta_data_widget = Paragraph::new(file_metadata_lines).block(file_metadata_block);

    frame.render_widget(file_meta_data_widget, layout[0]);

    let mut rows = vec![];
    for c in parquet_schema.columns() {
        let sort_order = c.sort_order().to_string();
        let row = match c.self_type() {
            parquet::schema::types::Type::PrimitiveType {
                basic_info,
                physical_type,
                type_length,
                scale,
                precision,
            } => {
                let ctype = basic_info.converted_type().to_string();
                let ltype = basic_info
                    .logical_type()
                    .map_or("".to_string(), |t| format!("{t:?}"));
                let ptype = physical_type.to_string();
                Row::new(vec![
                    basic_info.name().to_string(),
                    "primitive".to_string(),
                    ltype,
                    ctype,
                    ptype,
                    type_length.to_string(),
                    scale.to_string(),
                    precision.to_string(),
                    sort_order,
                ])
            }
            parquet::schema::types::Type::GroupType { basic_info, .. } => {
                let ctype = basic_info.converted_type().to_string();
                let ltype = basic_info
                    .logical_type()
                    .map_or("".to_string(), |t| format!("{t:?}"));
                Row::new(vec![
                    basic_info.name().to_string(),
                    "group".to_string(),
                    ltype,
                    ctype,
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    sort_order,
                ])
            }
        };
        rows.push(row);
    }

    let table = Table::new(rows, [Constraint::Min(10); 9])
        .column_spacing(1)
        .fg(ThemeColor::Text)
        .block(parquet_schema_block);

    frame.render_widget(table, layout[1]);
}

enum ThemeColor {
    Pine,
    Foam,
    Text,
    HighlightLow,
    Iris,
    Gold,
    Rose,
    Love,
}

impl From<ThemeColor> for Color {
    fn from(value: ThemeColor) -> Self {
        match value {
            ThemeColor::Pine => Self::Rgb(62, 143, 176),
            ThemeColor::Foam => Self::Rgb(156, 207, 216),
            ThemeColor::Text => Self::Rgb(224, 222, 244),
            ThemeColor::HighlightLow => Self::Rgb(42, 40, 62),
            ThemeColor::Iris => Self::Rgb(196, 167, 231),
            ThemeColor::Gold => Self::Rgb(246, 193, 119),
            ThemeColor::Rose => Self::Rgb(234, 154, 151),
            ThemeColor::Love => Self::Rgb(235, 111, 146),
        }
    }
}
