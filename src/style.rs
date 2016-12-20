use webrender_traits::{BorderSide, BoxShadowClipMode, ClipRegion, ColorF, DisplayListBuilder};
use webrender_traits::{GradientStop, LayoutPoint, LayoutRect, LayoutSize, BorderRadius};

pub struct BuildInfo {
    pub rect: LayoutRect,
    pub clip: ClipRegion,
    pub border_radius: Option<BorderRadius>,
}

/// Stores information for different styles that can be applied to objects
pub enum Style {
    /// Color
    BackgroundColor(ColorF),
    /// Color, Offset, Blur Radies, Spread Radius, Border Radius
    BoxShadow(ColorF, LayoutPoint, f32, f32, f32),
    /// Left, Top, Right, Bottom
    Border(BorderSide, BorderSide, BorderSide, BorderSide),
    /// Start Point, Stop Point, Gradient Stops
    Gradient(LayoutPoint, LayoutPoint, Vec<GradientStop>),
}

fn shadow_bounds(content_rect: &LayoutRect, blur_radius: f32, spread_radius: f32) -> LayoutRect {
    let inflation = spread_radius + blur_radius * 3.0;
    content_rect.inflate(inflation, inflation)
}

impl Style {
    pub fn build(&self, builder: &mut DisplayListBuilder, build_info: &BuildInfo) {
        let border_radius = build_info.border_radius.unwrap_or(BorderRadius::zero());
        match *self {
            Style::BackgroundColor(color) => {
                builder.push_rect(build_info.rect, build_info.clip, color);
            },
            Style::BoxShadow(color, offset, blur_radius, spread_radius, border_radius) => {
                builder.push_box_shadow(shadow_bounds(&build_info.rect.translate(&offset), blur_radius, spread_radius),
                                        ClipRegion::simple(&shadow_bounds(&build_info.rect.translate(&offset), blur_radius, spread_radius)),
                                        build_info.rect, // Box Bounds??
                                        offset,
                                        color,
                                        blur_radius,
                                        spread_radius,
                                        border_radius,
                                        BoxShadowClipMode::Outset);
            },
            Style::Border(border_left, border_top, border_right, border_bottom) => {
                builder.push_border(build_info.rect,
                                    build_info.clip,
                                    border_left,
                                    border_top,
                                    border_right,
                                    border_bottom,
                                    border_radius);
            },
            Style::Gradient(start_point, stop_point, ref gradient_stops) => {
                builder.push_gradient(build_info.rect,
                                      build_info.clip,
                                      start_point,
                                      stop_point,
                                      gradient_stops.clone());
            },
        }
    }
}