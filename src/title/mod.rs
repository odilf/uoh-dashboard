use rand::seq::IndexedRandom as _;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::{Line, Span, Text},
    widgets::Widget,
};

pub const TEXTS: &[&'static str] = &[
    include_str!("./text/big-money.txt"),
    include_str!("./text/big-mono-12.txt"),
    include_str!("./text/big.txt"),
    include_str!("./text/blur-vision.txt"),
    include_str!("./text/broadway.txt"),
    include_str!("./text/chiseled.txt"),
    include_str!("./text/crawford2.txt"),
    include_str!("./text/dancing-font.txt"),
    include_str!("./text/doh.txt"),
    include_str!("./text/impossible.txt"),
    include_str!("./text/isometric.txt"),
    include_str!("./text/rebel.txt"),
    include_str!("./text/terrace.txt"),
];

#[derive(Debug, Clone)]
struct Title {
    content: Text<'static>,
    width: u8,
    height: u8,
}

impl Title {
    pub fn new(text: &'static str) -> Self {
        let mut width = 0;
        let mut height = 0;
        for line in text.lines() {
            width = width.max(line.chars().count() as u8);
            height += 1;
        }

        Title {
            content: text.into(),
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Titles {
    titles: Vec<Title>,
    pub width: u8,
    pub height: u8,
}

impl Titles {
    pub fn new(texts: &[&'static str]) -> Self {
        let mut titles = Vec::with_capacity(texts.len());
        let mut height = 0;
        let mut width = 0;
        for text in TEXTS {
            let title = Title::new(text);
            height = title.height.max(height);
            width = title.width.max(width);
            titles.push(title);
        }

        Self {
            titles,
            width,
            height,
        }
    }
}

impl Default for Titles {
    fn default() -> Self {
        Self::new(TEXTS)
    }
}

impl Widget for &mut Titles {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.titles
            // TODO: Keep track of shown titles to have a more uniform distribution
            .choose(&mut rand::rng())
            .unwrap()
            .render(area, buf)
    }
}

impl Widget for &Title {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self.content).render(
            area.centered(
                Constraint::Length(self.width as u16),
                Constraint::Length(self.height as u16),
            ),
            buf,
        );
    }
}
