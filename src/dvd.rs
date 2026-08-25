use std::{
    f32::consts::TAU,
    time::{Duration, Instant},
};

use glam::Vec2;
use ratatui::{
    layout::{Margin, Rect},
    prelude::Buffer,
    style::Color,
    widgets::Widget,
};

use crate::title::Titles;

#[derive(Debug, Clone)]
pub struct Dvd {
    position: Vec2,
    velocity: Vec2,
    titles: Titles,
    last_render: Option<Instant>,
}

impl Dvd {
    pub fn new(velocity: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            velocity,
            titles: Titles::default(),
            last_render: None,
        }
    }

    pub fn inner_area(&self, area: Rect) -> Rect {
        area.inner(Margin::new(
            self.titles.active().width as u16 / 2,
            self.titles.active().height as u16 / 2,
        ))
    }

    pub fn update(&mut self, area: Rect, delta_time: Duration) {
        // Clamp position (if font changed or if screen resized)
        let area = self.inner_area(area);
        self.position.x = self
            .position
            .x
            .clamp(area.left() as f32, area.right() as f32);
        self.position.y = self
            .position
            .y
            .clamp(area.top() as f32, area.bottom() as f32);

        let velocity = self.velocity * delta_time.as_secs_f32() * 20.0;
        self.position += velocity;

        let min = Vec2::new(area.left() as f32, area.top() as f32);
        let max = Vec2::new(area.right() as f32, area.bottom() as f32);

        let below = (min - self.position).max(Vec2::ZERO);
        let above = (self.position - max).max(Vec2::ZERO);

        self.position += 2.0 * (below - above);

        let bounced = below.cmpgt(Vec2::ZERO) | above.cmpgt(Vec2::ZERO);
        self.velocity = Vec2::select(bounced, -self.velocity, self.velocity);
        if bounced.any() {
            self.titles.select_random();
            self.titles
                .active_mut()
                .set_color(Color::Indexed(rand::random()));

            self.velocity = self.velocity.rotate_angle(rand::random_range(-0.1..=0.1));
        }
    }
}

impl Widget for &mut Dvd {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(prev_t) = self.last_render {
            let delta = Instant::now().duration_since(prev_t);
            self.update(area, delta);
        } else {
            self.position = Vec2::new(
                area.x as f32 + area.width as f32 / 2.0,
                area.y as f32 + area.height as f32 / 2.0,
            );
        }

        let area = Rect::new(
            (self.position.x - self.titles.active().width as f32 / 2.0) as u16,
            (self.position.y - self.titles.active().height as f32 / 2.0) as u16,
            self.titles.active().width as u16,
            self.titles.active().height as u16,
        );
        self.titles.active().render(area, buf);
        self.last_render = Some(Instant::now());
    }
}

impl Default for Dvd {
    fn default() -> Self {
        Self::new(Vec2::from_angle(rand::random_range(0.0..TAU)))
    }
}
