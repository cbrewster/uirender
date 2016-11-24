use webrender_traits::DisplayListBuilder;

pub trait Renderable {
    fn render(&self, builder: &mut DisplayListBuilder);
}
