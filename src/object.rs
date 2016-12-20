use style::{BuildInfo, Style};
use webrender_traits::{BorderRadius, ClipRegion, ComplexClipRegion, DisplayListBuilder, LayoutRect};

pub struct Object {
    styles: Vec<Style>,
    rect: LayoutRect,
    border_radius: Option<BorderRadius>,
}

impl Object {
    pub fn new(rect: LayoutRect) -> Object {
        Object {
            styles: Vec::new(),
            rect: rect,
            border_radius: None,
        }
    }

    pub fn set_border_radius(&mut self, radius: BorderRadius) {
        self.border_radius = Some(radius);
    }

    pub fn remove_border_radius(&mut self) {
        self.border_radius = None;
    }

    pub fn build(&self, builder: &mut DisplayListBuilder) {
        let clip_region = match self.border_radius {
            Some(border_radius) => {
                let complex = ComplexClipRegion::new( self.rect, border_radius);
                builder.new_clip_region(&self.rect, vec![complex], None)
            },
            None => {
                ClipRegion::simple(&self.rect)
            }
        };

        let build_info = BuildInfo {
            rect: self.rect,
            clip: clip_region,
            border_radius: self.border_radius,
        };
        for style in &self.styles {
            style.build(builder, &build_info);
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}