use std::time::Duration;

use crossterm::event::{self, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Style, Stylize as _},
    widgets::Block,
};

use crate::title::Titles;

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
        if !event::poll(Duration::from_secs(1))? {
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
