use arrow::{
    array::RecordBatch,
    error::ArrowError,
    util::{
        display::{ArrayFormatter, FormatOptions},
        pretty::pretty_format_batches,
    },
};
use parquet::file::statistics::Statistics;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Offset, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, Paragraph, Row, Table, Tabs},
    Frame,
};

use super::state::State;
use crate::error::Result;

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
        .constraints(vec![
            Constraint::Length(17),
            Constraint::Min(0),
            Constraint::Percentage(50),
        ])
        .split(screen[0]);

    let tabs = Tabs::new(Tab::get_headers().to_vec())
        .highlight_style(Style::default().fg(ThemeColor::Iris.into()))
        .select(state.tab as usize)
        .divider("|")
        .padding(" ", " ");

    frame.render_widget(tabs, title_line[0]);
    frame.render_widget(" (Tab) ".fg(ThemeColor::Subtle), title_line[1]);

    let label = Paragraph::new(Line::from(vec![
        Span::from(" parqour ")
            .bg(ThemeColor::Foam)
            .fg(ThemeColor::HighlightLow),
        Span::from(format!(" {} ", &state.viewer.file_stem)).bg(ThemeColor::Pine),
    ]))
    .right_aligned();
    frame.render_widget(label, title_line[2]);

    match state.tab {
        Tab::Data => {
            render_data(state, frame, screen[1]);
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
    let kv_n = state.viewer.file_kv_data.len();
    let metadata_height = if kv_n == 0 { 4 } else { 5 + kv_n } as u16;
    let layout = Layout::vertical([
        Constraint::Length(metadata_height),
        Constraint::Min(5),
        Constraint::Length(5),
    ])
    .split(rect);
    let file_metadata_block = Block::bordered()
        .title("File metadata".bold())
        .fg(ThemeColor::Subtle);

    let parquet_schema_block = Block::bordered()
        .title("Schema".bold())
        .title(Title::from("(↑/↓)").alignment(Alignment::Center))
        .fg(ThemeColor::Subtle);

    let mut file_metadata_lines = vec![
        Line::from(vec![
            Span::styled("Version: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(&state.viewer.version),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("Created by: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(&state.viewer.created_by),
        ]),
        Line::from(vec![
            Span::styled("# rows: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(state.viewer.num_rows.to_string()),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("# columns: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(state.viewer.num_cols.to_string()),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled(
                "# row groups ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(state.viewer.num_row_groups.to_string()),
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
            Constraint::Max(state.viewer.max_col_name_width as u16),
            // Constraint::Min(state.viewer.max_col_name_width as u16),
            Constraint::Max(11),
            Constraint::Min(12),
            Constraint::Max(16),
            Constraint::Max(20),
            Constraint::Max(11),
            Constraint::Max(5),
            Constraint::Max(10),
            Constraint::Max(10),
            // Constraint::Min(0),
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
    .highlight_style(Style::default().fg(ThemeColor::Iris.into()).bold())
    .block(parquet_schema_block);

    let col_selected = state.table_state.selected().unwrap();

    let chunk_meta = state.viewer.row_groups[state.chunk_ind].column(col_selected);
    let col_name = chunk_meta.column_descr().name();
    let num_values = chunk_meta.num_values().to_string();
    let encodings = chunk_meta
        .encodings()
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let compression = chunk_meta.compression().to_string();
    let compressed_size = chunk_meta.compressed_size() / 1000;
    let uncompressed_size = chunk_meta.uncompressed_size() / 1000;

    let mut chunk_metadata_lines = vec![
        Line::from(vec![
            Span::styled("# values: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(&num_values),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("Encodings: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(&encodings),
        ]),
        Line::from(vec![
            Span::styled(
                "Compression: ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(compression),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("Compressed: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(format!("{}kB", compressed_size)),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled(
                "Uncompressed: ",
                Style::default().fg(ThemeColor::Love.into()),
            ),
            Span::raw(format!("{}kB", uncompressed_size)),
        ]),
    ];

    if let Some(stats) = chunk_meta.statistics() {
        let distinct_cnt = stats
            .distinct_count_opt()
            .map_or("".to_string(), |v| v.to_string());
        let null_cnt = stats
            .null_count_opt()
            .map_or("".to_string(), |v| v.to_string());
        let (min, max) = match stats {
            Statistics::Boolean(value_stats) => {
                let min_str = value_stats
                    .min_opt()
                    .map_or("".to_string(), |v| v.to_string());
                let max_str = value_stats
                    .max_opt()
                    .map_or("".to_string(), |v| v.to_string());
                (min_str, max_str)
            }
            Statistics::Int32(value_stats) => {
                let min_str = value_stats
                    .min_opt()
                    .map_or("".to_string(), |v| v.to_string());
                let max_str = value_stats
                    .max_opt()
                    .map_or("".to_string(), |v| v.to_string());
                (min_str, max_str)
            }
            Statistics::Int64(value_stats) => {
                let min_str = value_stats
                    .min_opt()
                    .map_or("".to_string(), |v| v.to_string());
                let max_str = value_stats
                    .max_opt()
                    .map_or("".to_string(), |v| v.to_string());
                (min_str, max_str)
            }
            Statistics::Int96(value_stats) => {
                let min_str = value_stats
                    .min_opt()
                    .map_or("".to_string(), |v| v.to_string());
                let max_str = value_stats
                    .max_opt()
                    .map_or("".to_string(), |v| v.to_string());
                (min_str, max_str)
            }
            Statistics::Float(value_stats) => {
                let min_str = value_stats
                    .min_opt()
                    .map_or("".to_string(), |v| v.to_string());
                let max_str = value_stats
                    .max_opt()
                    .map_or("".to_string(), |v| v.to_string());
                (min_str, max_str)
            }
            Statistics::Double(value_stats) => {
                let min_str = value_stats
                    .min_opt()
                    .map_or("".to_string(), |v| v.to_string());
                let max_str = value_stats
                    .max_opt()
                    .map_or("".to_string(), |v| v.to_string());
                (min_str, max_str)
            }
            Statistics::ByteArray(value_stats) => {
                let min_str = value_stats.min_opt().map_or("".to_string(), |v| {
                    std::str::from_utf8(v.data()).unwrap().to_string()
                });
                let max_str = value_stats.max_opt().map_or("".to_string(), |v| {
                    std::str::from_utf8(v.data()).unwrap().to_string()
                });
                (min_str, max_str)
            }
            Statistics::FixedLenByteArray(value_stats) => {
                let min_str = value_stats.min_opt().map_or("".to_string(), |v| {
                    std::str::from_utf8(v.data()).unwrap().to_string()
                });
                let max_str = value_stats.max_opt().map_or("".to_string(), |v| {
                    std::str::from_utf8(v.data()).unwrap().to_string()
                });
                (min_str, max_str)
            }
        };
        let l = Line::from(vec![
            Span::styled("Min: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(min),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("Max: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(max),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("# distinct: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(distinct_cnt),
            Span::styled("  |  ", Style::default().fg(ThemeColor::Rose.into())),
            Span::styled("# null: ", Style::default().fg(ThemeColor::Love.into())),
            Span::raw(null_cnt),
        ]);
        chunk_metadata_lines.push(l);
    };

    let chunk_block = Block::bordered()
        .title(
            Line::from(vec![
                Span::raw("Chunk metadata: "),
                Span::styled(
                    format!("{}[{}]", col_name, state.chunk_ind),
                    Style::default().fg(ThemeColor::Iris.into()),
                ),
            ])
            .bold(),
        )
        .title(Title::from("(←/→)").alignment(Alignment::Center))
        .fg(ThemeColor::Subtle);

    let p = Paragraph::new(chunk_metadata_lines)
        .block(chunk_block)
        .fg(ThemeColor::Text);
    frame.render_stateful_widget(table, layout[1], &mut state.table_state);
    frame.render_widget(p, layout[2]);
}

fn render_data(state: &mut State, frame: &mut Frame, rect: Rect) -> Result<()> {
    let col_names: Vec<_> = state
        .viewer
        .batch
        .schema_ref()
        .fields()
        .iter()
        .skip(state.viewer.col_offset)
        .map(|f| f.name().to_owned())
        .take(state.viewer.visible_cols)
        .collect();
    let table_slice = batch_slice(
        &state.viewer.batch,
        state.viewer.row_offset,
        state.viewer.col_offset,
        state.viewer.visible_rows,
        state.viewer.visible_cols,
    );

    let col_width = 10_usize;
    let col_constraints =
        Constraint::from_lengths(vec![col_width as u16; state.viewer.visible_cols]);
    let col_layout = Layout::horizontal(col_constraints).split(rect);
    for (i, (data, name)) in table_slice.iter().zip(col_names).enumerate() {
        let mut lines = vec![Line::from(mask_string(&name, col_width - 1)).fg(ThemeColor::Love)];
        lines.extend(data.iter().enumerate().map(|(j, c)| {
            let bg_color = if j % 2 == 0 {
                ThemeColor::HighlightLow
            } else {
                ThemeColor::Base
            };
            let fg_color = if ((i + state.viewer.col_offset) == state.viewer.selected_col)
                && ((j + state.viewer.row_offset) == state.viewer.selected_row)
            {
                ThemeColor::Iris
            } else {
                ThemeColor::Text
            };
            let s = mask_string(c.as_str(), col_width - 1);
            Line::from(format!("{s} "))
                .bg(bg_color)
                .fg(fg_color)
                .alignment(Alignment::Right)
        }));
        let col = Text::from(lines);
        frame.render_widget(col, col_layout[i]);
    }

    Ok(())
}

fn mask_string(s: &str, max_len: usize) -> &str {
    if s.chars().count() > max_len {
        let mut char_indices = s.char_indices();
        let truncate_at = char_indices.nth(max_len).map_or(s.len(), |(idx, _)| idx);
        &s[..truncate_at]
    } else {
        s
    }
}
fn batch_slice(
    batch: &RecordBatch,
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
) -> Vec<Vec<String>> {
    let options = FormatOptions::default();
    batch
        .columns()
        .iter()
        .skip(col)
        .map(|c| {
            let formatter = ArrayFormatter::try_new(c.as_ref(), &options).unwrap();
            (row..(row + nrows))
                .map(|i| formatter.value(i).to_string())
                .collect::<Vec<_>>()
        })
        .take(ncols)
        .collect()
}

fn render_batch(batch: &RecordBatch, row: usize, col: usize, nrows: usize, ncols: usize) {
    let col_names: Vec<_> = batch
        .schema()
        .fields()
        .iter()
        .skip(col)
        .map(|f| f.name())
        .take(ncols)
        .collect();
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
