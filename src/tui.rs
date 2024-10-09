use chrono::{Duration, Local, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::{io, iter::repeat, sync::Arc};

use crate::{
    kanata_tcp::{disable_keyboards, enable_keyboards},
    persistance::{delete_persisted_time, persist_time},
    shared_data::SharedData,
    symlinks::{lock_taskdescription, unlock_taskdescription},
    tui_basic,
};

pub fn tui_main(shared_data: Arc<SharedData>) -> io::Result<()> {
    let mut terminal = tui_basic::init()?;
    let app_result = App::new(shared_data).run(&mut terminal);
    tui_basic::restore()?;
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    shared_data: Arc<SharedData>,
}

impl App {
    fn new(shared_data: Arc<SharedData>) -> Self {
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
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
            ])
            .split(frame.area());
        render_status(self, chunks[0], frame.buffer_mut());
        render_symlinks(self, chunks[1], frame.buffer_mut());
        render_clients(self, chunks[2], frame.buffer_mut());
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
            KeyCode::Char('q') => {
                delete_persisted_time();
                self.exit();
            }
            KeyCode::Enter => {
                let mut times = self.shared_data.times.lock().unwrap();
                // no times stored => timer has not yet been started
                if times.is_none() {
                    let now = Utc::now();
                    let duration =
                        Duration::minutes(self.shared_data.config.timer_duration_minutes);
                    *times = Some((now, duration));

                    unlock_taskdescription(self.shared_data.clone());
                    persist_time(now, duration);
                }
            }
            KeyCode::Char('+') => {
                let mut times = self.shared_data.times.lock().unwrap();
                if times.is_some() {
                    let (start_time, duration) = times.unwrap();
                    let new_duration = duration + Duration::minutes(1);
                    *times = Some((start_time, new_duration));
                    persist_time(start_time, new_duration);
                }
            }
            KeyCode::Char('-') => {
                let mut times = self.shared_data.times.lock().unwrap();
                if times.is_some() {
                    let (start_time, duration) = times.unwrap();
                    let new_duration = duration - Duration::minutes(1);
                    *times = Some((start_time, new_duration));
                    persist_time(start_time, new_duration);
                }
            }
            KeyCode::Char(' ') => {
                disable_keyboards(self.shared_data.clone());
            }
            KeyCode::Esc => {
                enable_keyboards(self.shared_data.clone());
            }
            KeyCode::Char('d') => {
                lock_taskdescription(self.shared_data.clone());
            }
            KeyCode::Char('r') => {
                unlock_taskdescription(self.shared_data.clone());
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn render_status(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" Timer ".bold());
    let instructions = match *app.shared_data.times.lock().unwrap() {
        Some(_) => Title::from(vec![
            " +1min".into(),
            " <+> ".blue().bold(),
            " -1min".into(),
            " <-> ".blue().bold(),
        ]),
        None => Title::from(vec![" Start exam".into(), " <Enter> ".blue().bold()]),
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
                start_time
                    .with_timezone(&Local)
                    .format("%H:%M:%S")
                    .to_string()
                    .yellow()
                    .bold(),
                "  Finish: ".into(),
                (start_time + duration)
                    .with_timezone(&Local)
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

fn render_symlinks(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" Status ".bold());
    let instructions = Title::from(vec![
        " Set to dummy".into(),
        " <d> ".blue().bold(),
        " Set to real".into(),
        " <r> ".blue().bold(),
    ]);
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let counter_text = match app.shared_data.symlink_target.lock().unwrap().as_ref() {
        Some(path) => Line::from(vec![
            "Currently pointing to ".into(),
            path.clone().yellow().bold(),
        ]),
        None => Line::from(vec![
            "Symlink ".into(),
            app.shared_data.symlink_info.symlink_path.clone().yellow(),
            " is not accessible".into(),
        ]),
    };

    Paragraph::new(counter_text)
        //.centered()
        .block(block)
        .render(area, buf);
}

fn render_clients(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" Client Overview ".bold());
    let instructions = Title::from(vec![
        " Quit ".into(),
        "<q> ".blue().bold(),
        "  Enable keys ".into(),
        "<Escape> ".blue().bold(),
        "  Disable keys ".into(),
        "<Space> ".blue().bold(),
    ]);
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        "kbd layer  name      IP address       time since timer request".bold(),
    ]));
    lines.append(
        &mut app
            .shared_data
            .clients
            .iter()
            .map(|client| {
                Line::from(vec![
                    try_pad_string(
                        match client.current_layer.lock().unwrap().as_ref() {
                            Some(layer) => layer.clone(),
                            None => "  ---".to_string(),
                        },
                        ' ',
                        11,
                    )
                    .yellow(),
                    try_pad_string(client.name.clone(), ' ', 10).into(),
                    try_pad_string(client.ip_address.to_string(), ' ', 18).into(),
                    match client.last_timer_access.lock().unwrap().as_ref() {
                        Some(last_access) => {
                            let duration = Utc::now() - last_access;
                            format!(
                                "{}:{:02}min",
                                duration.num_minutes(),
                                duration.num_seconds() % 60
                            )
                        }
                        None => " -".to_string(),
                    }
                    .into(),
                ])
            })
            .collect::<Vec<_>>(),
    );
    if app.shared_data.config.tui_show_nonclient_timer_accesses {
        lines.append(
            &mut app
                .shared_data
                .nonclients
                .lock()
                .unwrap()
                .iter()
                .map(|nonclient| {
                    Line::from(vec![
                        "                     ".into(),
                        try_pad_string(nonclient.ip_address.to_string(), ' ', 18).into(),
                        {
                            let duration = Utc::now() - nonclient.last_timer_access;
                            format!(
                                "{}:{:02}min",
                                duration.num_minutes(),
                                duration.num_seconds() % 60
                            )
                        }
                        .into(),
                    ])
                })
                .collect::<Vec<_>>(),
        );
    }
    let counter_text = Text::from(lines);

    Paragraph::new(counter_text)
        //.centered()
        .block(block)
        .render(area, buf);
}

fn try_pad_string(mut string: String, pad_char: char, desired_length: usize) -> String {
    let pad_len = desired_length as isize - string.len() as isize;
    if pad_len > 0 {
        let padding = repeat(pad_char).take(pad_len as usize).collect::<String>();
        string.push_str(&padding);
    }
    string
}
