extern crate uirender;

use uirender::window::Window;
use uirender::primitives::Rectangle;
use uirender::units::{LayoutRect, LayoutPoint, LayoutSize};

fn main() {
    let mut window = Window::new("Hello World!");
    let rectangle = Rectangle::new(LayoutRect::new(LayoutPoint::new(0.0, 0.0), LayoutSize::new(100.0, 100.0)));
    window.add_renderable(rectangle);
    window.run();
}
