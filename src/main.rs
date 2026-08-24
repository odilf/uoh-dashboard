use std::fs;

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};
use tracing::Level;

use crate::{config::Config, dvd::Dvd, title::Titles};

mod config;
mod dvd;
mod title;
mod weather;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let subscriber = tracing_subscriber::fmt();

    #[cfg(debug_assertions)]
    let subscriber = subscriber
        .with_writer(
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("./uoh-dashboard.log")?,
        )
        .with_max_level(Level::DEBUG);

    #[cfg(not(debug_assertions))]
    let subscriber = subscriber
        .with_writer(std::io::stderr)
        .with_max_level(Level::INFO);

    subscriber.init();

    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state = State::default();
    loop {
        terminal.draw(|frame| state.render(frame))?;

        if !event::poll(state.config.base_refresh_period.into())? {
            continue;
        }
        match event::read()? {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Char('q') => break Ok(()),
                _ => (),
            },
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    mode: Mode,
    config: Config,
    titles: Titles,
    dvd: Dvd,
}

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    Dvd,
    Stats,
}

impl State {
    pub fn render(&mut self, frame: &mut Frame) {
        match self.mode {
            Mode::Dvd => frame.render_widget(&mut self.dvd, frame.area()),
            Mode::Stats => {
                let [top, middle, bottom] = frame.area().layout(&Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(self.titles.height as u16),
                    Constraint::Fill(1),
                ]));

                let [left, middle, right] = middle.layout(&Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(self.titles.width as u16),
                    Constraint::Fill(1),
                ]));

                frame.render_widget(&mut self.titles, middle);
            }
        }
    }
}
