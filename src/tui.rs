use chrono::{Duration, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io;

use crate::{shared_data::SharedData, tui_basic};

pub fn tui_main(shared_data: SharedData) -> io::Result<()> {
    let mut terminal = tui_basic::init()?;
    let app_result = App::new(shared_data).run(&mut terminal);
    tui_basic::restore()?;
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    shared_data: SharedData,
}

impl App {
    fn new(shared_data: SharedData) -> Self {
        App {
            exit: false,
            shared_data,
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui_basic::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5)])
            .split(frame.area());
        render_status(self, chunks[0], frame.buffer_mut());
        render_clients(self, chunks[1], frame.buffer_mut());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        if crossterm::event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => {
                let mut times = self.shared_data.times.lock().unwrap();
                // no times stored => timer has not yet been started
                if times.is_none() {
                    let now = Utc::now();
                    let duration = Duration::seconds(60 * 90);
                    *times = Some((now, duration));
                }
            }
            KeyCode::Char('+') => {
                let mut times = self.shared_data.times.lock().unwrap();
                if times.is_some() {
                    let (start_time, duration) = times.unwrap();
                    *times = Some((start_time, duration + Duration::minutes(1)))
                }
            }
            KeyCode::Char('-') => {
                let mut times = self.shared_data.times.lock().unwrap();
                if times.is_some() {
                    let (start_time, duration) = times.unwrap();
                    *times = Some((start_time, duration - Duration::minutes(1)))
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn render_status(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" Status ".bold());
    let instructions = match *app.shared_data.times.lock().unwrap() {
        Some(_) => Title::from(vec![
            " +1min".into(),
            " <+> ".blue().bold(),
            " -1min".into(),
            " <-> ".blue().bold(),
        ]),
        None => Title::from(vec![" Start Timer".into(), " <Enter> ".blue().bold()]),
    };
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let counter_text = match *app.shared_data.times.lock().unwrap() {
        Some((start_time, duration)) => {
            let dur = duration - (Utc::now() - start_time);
            Line::from(vec![
                "Start: ".into(),
                start_time.format("%H:%M:%S").to_string().yellow().bold(),
                "  Finish: ".into(),
                (start_time + duration)
                    .format("%H:%M:%S")
                    .to_string()
                    .yellow()
                    .bold(),
                "  Duration: ".into(),
                duration.num_minutes().to_string().yellow().bold(),
                "min".yellow().bold(),
                "  Time left: ".into(),
                format!("{:02}", dur.num_minutes()).yellow().bold(),
                ":".yellow().bold(),
                format!("{:02}", dur.num_seconds() % 60).yellow().bold(),
                "min".yellow().bold(),
            ])
        }
        None => Line::from("INACTIVE".red().bold()),
    };

    Paragraph::new(counter_text)
        //.centered()
        .block(block)
        .render(area, buf);
}

fn render_clients(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" Client Overview ".bold());
    let instructions = Title::from(vec![" - ".into(), "<Tab> ".blue().bold()]);
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let lines = app
        .shared_data
        .clients
        .lock()
        .unwrap()
        .iter()
        .map(|client| {
            Line::from(vec![
                if client.is_online {
                    "online   ".green()
                } else {
                    "offline  ".red()
                },
                client.ip_address.to_string().into(),
                "   ".into(),
                match &client.current_layer {
                    Some(layer) => format!("switched keyboard to \"{}\" last time", layer).into(),
                    None => "keyboard never changed".into(),
                },
            ])
        })
        .collect::<Vec<_>>();
    let counter_text = Text::from(lines);

    Paragraph::new(counter_text)
        //.centered()
        .block(block)
        .render(area, buf);
}
