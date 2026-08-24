use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Text,
    widgets::Widget,
};

const TEXTS: &[&str] = &[
    include_str!("./text/rebel.txt"),
    // Other
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
    include_str!("./text/terrace.txt"),
];

#[derive(Debug, Clone)]
pub struct Title {
    content: Text<'static>,
    pub width: u8,
    pub height: u8,
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

    pub(crate) fn set_color(&mut self, color: Color) {
        self.content.style = Style::default().fg(color);
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(TEXTS[0])
    }
}

#[derive(Debug, Clone)]
pub struct Titles {
    titles: Vec<Title>,
    current: u8,
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
            current: 0,
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

impl Titles {
    pub fn select_random(&mut self) {
        // TODO: Keep track of shown titles to have a more uniform distribution
        self.current = rand::random_range(0..self.titles.len()) as u8;
    }

    pub fn active(&self) -> &Title {
        &self.titles[self.current as usize]
    }
    pub fn active_mut(&mut self) -> &mut Title {
        &mut self.titles[self.current as usize]
    }
}

impl Widget for &mut Titles {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.select_random();
        self.active().render(area, buf)
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
