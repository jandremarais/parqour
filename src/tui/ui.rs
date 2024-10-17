use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Row, Table, Tabs},
    Frame,
};

use super::state::State;

pub fn render(state: &mut State, frame: &mut Frame) {
    frame.render_widget(
        Block::new().bg(ThemeColor::Base).fg(ThemeColor::Text),
        frame.area(),
    );

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
        // .fg(ThemeColor::Text)
        .highlight_style(Style::default().fg(ThemeColor::Iris.into()))
        .select(state.tab as usize)
        .divider("|")
        .padding(" ", " ");

    frame.render_widget(tabs, title_line[0]);
    let label = Paragraph::new(Line::from(vec![
        Span::from(" parqour ")
            .bg(ThemeColor::Foam)
            .fg(ThemeColor::HighlightLow),
        Span::from(format!(" {} ", &state.viewer.file_stem)).bg(ThemeColor::Pine),
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
    // let parquet_metadata = state.viewer.parquet_metadata();
    // let file_metadata = parquet_metadata.file_metadata();
    // let parquet_schema = file_metadata.schema_descr();

    let metadata_height = (6 + state.viewer.file_kv_data.len() + 1) as u16;
    let schema_height = 3 + state.viewer.num_cols as u16;
    let layout = Layout::vertical([
        Constraint::Length(metadata_height),
        Constraint::Length(schema_height),
    ])
    .split(rect);
    let file_metadata_block = Block::bordered()
        .title("File metadata".bold())
        .fg(ThemeColor::Subtle);

    let parquet_schema_block = Block::bordered()
        .title("Schema".bold())
        .fg(ThemeColor::Subtle);

    let mut file_metadata_lines = vec![
        Line::from(vec![
            Span::styled("Version: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(&state.viewer.version),
        ]),
        Line::from(vec![
            Span::styled(
                "Number of rows: ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(state.viewer.num_rows.to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Number of columns: ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(state.viewer.num_cols.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Created by: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(&state.viewer.created_by),
        ]),
    ];
    if !state.viewer.file_kv_data.is_empty() {
        file_metadata_lines.push(Line::from(vec![Span::styled(
            "Other",
            Style::default().fg(ThemeColor::Rose.into()).bold(),
        )]));

        for (k, v) in state.viewer.file_kv_data.iter() {
            let l = Line::from(vec![
                Span::styled(
                    format!("{}: ", k),
                    Style::default().fg(ThemeColor::Love.into()),
                ),
                Span::raw(v),
            ]);
            file_metadata_lines.push(l);
        }
    };
    let file_meta_data_widget = Paragraph::new(file_metadata_lines)
        .block(file_metadata_block)
        .fg(ThemeColor::Text);

    frame.render_widget(file_meta_data_widget, layout[0]);

    let mut rows = vec![];
    for (i, r) in state.viewer.schema_table_data.iter().enumerate() {
        let bg_color = if i % 2 == 0 {
            ThemeColor::HighlightLow
        } else {
            ThemeColor::Base
        };
        let row = Row::new(r.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        rows.push(row.fg(ThemeColor::Text).bg(bg_color));
    }

    let table = Table::new(
        rows,
        [
            Constraint::Min(state.viewer.max_col_name_width as u16),
            Constraint::Min(11),
            Constraint::Min(12),
            Constraint::Min(14),
            Constraint::Min(13),
            Constraint::Min(4),
            Constraint::Min(5),
            Constraint::Min(9),
            Constraint::Min(10),
        ],
    )
    .column_spacing(1)
    .header(
        Row::new(vec![
            "Name",
            "Column type",
            "Logical type",
            "Converted type",
            "Physical type",
            "Type length",
            "Scale",
            "Precision",
            "Sort order",
        ])
        .fg(ThemeColor::Love),
    )
    .block(parquet_schema_block);

    frame.render_widget(table, layout[1]);
}

#[allow(dead_code)]
enum ThemeColor {
    Base,
    Surface,
    Pine,
    Foam,
    Text,
    Subtle,
    HighlightLow,
    HighlightMed,
    Iris,
    Gold,
    Rose,
    Love,
}

impl From<ThemeColor> for Color {
    fn from(value: ThemeColor) -> Self {
        match value {
            ThemeColor::Base => Self::Rgb(35, 33, 54),
            ThemeColor::Surface => Self::Rgb(42, 39, 63),
            ThemeColor::Pine => Self::Rgb(62, 143, 176),
            ThemeColor::Foam => Self::Rgb(156, 207, 216),
            ThemeColor::Text => Self::Rgb(224, 222, 244),
            ThemeColor::Subtle => Self::Rgb(144, 140, 170),
            ThemeColor::HighlightLow => Self::Rgb(42, 40, 62),
            ThemeColor::HighlightMed => Self::Rgb(68, 65, 90),
            ThemeColor::Iris => Self::Rgb(196, 167, 231),
            ThemeColor::Gold => Self::Rgb(246, 193, 119),
            ThemeColor::Rose => Self::Rgb(234, 154, 151),
            ThemeColor::Love => Self::Rgb(235, 111, 146),
        }
    }
}
