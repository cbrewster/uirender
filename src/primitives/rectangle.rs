use euclid::Rect;
use primitives::Renderable;
use webrender_traits::{ClipRegion, ColorF, DisplayListBuilder};

pub struct Rectangle {
    pub rect: Rect<f32>,
}

impl Rectangle {
    pub fn new(rect: Rect<f32>) -> Rectangle {
        Rectangle {
            rect: rect,
        }
    }
}

impl Renderable for Rectangle {
    fn render(&self, builder: &mut DisplayListBuilder) {
        builder.push_rect(self.rect, ClipRegion::simple(&self.rect), ColorF::new(1.0, 0.0, 0.0, 1.0));
    }
}
