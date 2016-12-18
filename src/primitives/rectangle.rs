use glutin::Event;
use primitives::Renderable;
use std::cell::Cell;
use webrender_traits::{ClipRegion, ColorF, DisplayListBuilder};
use webrender_traits::{LayoutPoint, LayoutRect};

pub struct Rectangle {
    pub rect: LayoutRect,
    dirty: Cell<bool>,
    color: ColorF,
    active: bool,
}

impl Rectangle {
    pub fn new(rect: LayoutRect) -> Rectangle {
        Rectangle {
            rect: rect,
            dirty: Cell::new(true),
            color: ColorF::new(1.0, 0.0, 0.0, 1.0),
            active: false,
        }
    }
}

impl Renderable for Rectangle {
    fn render(&self, builder: &mut DisplayListBuilder) {
        builder.push_rect(self.rect, ClipRegion::simple(&self.rect), self.color);
        self.dirty.set(false);
    }

    fn update(&mut self) {
        // self.rect = LayoutRect::new(self.rect.origin, LayoutSize::new(self.rect.size.width * 1.001, self.rect.size.height * 1.001));
        // self.dirty.set(true);
    }

    fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    fn handle_window_event(&mut self, event: &Event) {
        if let &Event::MouseMoved(x, y) = event {
            if self.rect.contains(&LayoutPoint::new((x / 2) as f32, (y / 2) as f32)) {
                self.color = ColorF::new(0.0, 1.0, 0.0, 1.0);
                if !self.active {
                    self.dirty.set(true);
                }
                self.active = true;
            } else {
                self.color = ColorF::new(1.0, 0.0, 0.0, 1.0);
                if self.active {
                    self.dirty.set(true);
                }
                self.active = false;
            }
        }
    }
}
