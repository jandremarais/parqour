use std::{io, path::PathBuf};

use arrow::util::pretty::print_batches;
use clap::Parser;
use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use parqour::tui;
use parquet::{
    arrow::arrow_reader::ParquetRecordBatchReaderBuilder,
    column::reader::ColumnReader,
    file::reader::{FileReader, SerializedFileReader},
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let Ok(file) = std::fs::File::open(&cli.file) else {
        panic!("file not found");
    };
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    dbg!(builder.metadata().num_row_groups());
    // builder.build()
    let mut rdr = builder.with_batch_size(8).build()?;
    let batch = rdr.next().unwrap()?;
    print_batches(&[batch])?;

    color_eyre::install()?;
    let mut terminal = tui::init()?;
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart  your temrinal to recover: {}",
            err
        );
    };
    app_result?;
    Ok(())
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the file you want to display
    file: PathBuf,
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handline key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) -> Result<()> {
        if self.counter == 0 {
            bail!("counter can't be negative");
        }
        self.counter -= 1;
        Ok(())
    }

    fn increment_counter(&mut self) -> Result<()> {
        if self.counter > 5 {
            bail!("counter overflow");
        }
        self.counter += 1;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" PARQOUR ".bold());
        let block = Block::bordered().title(title.alignment(Alignment::Center));
        let counter_text = Text::from(vec![Line::from(vec![
            "Value ".into(),
            self.counter.to_string().blue(),
        ])]);
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
