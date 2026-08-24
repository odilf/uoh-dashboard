use std::time::Duration;

use crossterm::event::{self, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Style, Stylize as _},
    widgets::Block,
};

use crate::{config::Config, title::Titles};

mod config;
mod title;
mod weather;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
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
    config: Config,
    titles: Titles,
}

impl State {
    pub fn render(&mut self, frame: &mut Frame) {
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
