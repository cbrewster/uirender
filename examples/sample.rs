extern crate uirender;
extern crate webrender_traits;

use uirender::window::Window;
use uirender::object::Object;
use uirender::style::Style;
use webrender_traits::{BorderRadius, BorderSide, BorderStyle, ColorF, LayoutRect, LayoutPoint, LayoutSize};

fn main() {
    let mut window = Window::new("uirender");
    for i in 0..5 {
        let mut object = Object::new(LayoutRect::new(LayoutPoint::new(10.0 + i as f32 * 130.0, 10.0), LayoutSize::new(100.0, 25.0)));
        object.add_style(Style::BackgroundColor(ColorF::new(0.95, 0.95, 0.95, 1.0)));
        let border = BorderSide {
            width: 1.0,
            color: ColorF::new(0.5, 0.5, 0.5, 1.0),
            style: BorderStyle::Solid,
        };
        object.add_style(Style::Border(border, border, border, border));
        object.add_style(Style::BoxShadow(ColorF::new(0.0, 0.0, 0.0, 0.2), LayoutPoint::zero(), 5.0, 1.0, 0.0));
        object.set_border_radius(BorderRadius::uniform(0.0));
        window.add_object(object);
    }
    window.run();
}
