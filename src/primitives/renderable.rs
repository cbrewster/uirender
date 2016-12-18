use glutin;
use webrender_traits::DisplayListBuilder;

pub trait Renderable {
    fn render(&self, builder: &mut DisplayListBuilder);
    fn update(&mut self);
    fn is_dirty(&self) -> bool;
    fn handle_window_event(&mut self, _event: &glutin::Event) {}
}
