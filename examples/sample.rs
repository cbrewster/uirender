extern crate euclid;
extern crate uirender;

use euclid::{Point2D, Rect, Size2D};
use uirender::window::Window;
use uirender::primitives::Rectangle;

fn main() {
    let mut window = Window::new("Hello World!");
    let rectangle = Rectangle::new(Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)));
    window.add_renderable(rectangle);
    window.run();
}
