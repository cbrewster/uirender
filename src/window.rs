use gleam::gl;
use glutin;
use primitives::Renderable;
use std::cell::RefCell;
use webrender::RendererOptions;
use webrender::renderer::Renderer;
use webrender_traits::{BorderRadius, ColorF, ComplexClipRegion, DisplayListBuilder};
use webrender_traits::{Epoch, MixBlendMode, PipelineId, RendererKind, RenderApi};
use webrender_traits::{RenderNotifier, ScrollPolicy};
use webrender_traits::{LayoutSize, LayoutPoint, LayoutRect, LayoutTransform, DeviceUintSize};

struct Notifier {
    window_proxy: glutin::WindowProxy,
}

impl Notifier {
    fn new(window_proxy: glutin::WindowProxy) -> Notifier {
        Notifier {
            window_proxy: window_proxy,
        }
    }
}

impl RenderNotifier for Notifier {
    fn new_frame_ready(&mut self) {
        self.window_proxy.wakeup_event_loop();
    }

    fn new_scroll_frame_ready(&mut self, _composite_needed: bool) {
        self.window_proxy.wakeup_event_loop();
    }

    fn pipeline_size_changed(&mut self,
                         _: PipelineId,
                         _: Option<LayoutSize>) {
    }
}

pub struct Window {
    window: glutin::Window,
    renderer: Renderer,
    api: RenderApi,
    pipeline_id: PipelineId,
    epoch: Epoch,
    size: DeviceUintSize,
    root_background_color: ColorF,
    renderables: Vec<Box<RefCell<Renderable>>>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        let window = glutin::WindowBuilder::new()
                     .with_title(title)
                     .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
                     .build()
                     .unwrap();

        unsafe {
            window.make_current().ok();
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        }

        println!("OpenGl version {}",  gl::get_string(gl::VERSION));

        let (width, height) = window.get_inner_size_pixels().unwrap();
        let dpi = window.hidpi_factor();

        let opts = RendererOptions {
            device_pixel_ratio: dpi,
            resource_override_path: None,
            enable_aa: false,
            enable_msaa: false,
            enable_profiler: false,
            enable_recording: false,
            enable_scrollbars: false,
            debug: false,
            precache_shaders: true,
            renderer_kind: RendererKind::Native,
            enable_subpixel_aa: false,
            clear_framebuffer: true,
            clear_empty_tiles: false,
            clear_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
        };

        let (renderer, sender) = Renderer::new(opts);
        let api = sender.create_api();

        let notifier = Box::new(Notifier::new(window.create_window_proxy()));
        renderer.set_render_notifier(notifier);

        Window {
            window: window,
            renderer: renderer,
            api: api,
            pipeline_id: PipelineId(0, 0),
            epoch: Epoch(0),
            size: DeviceUintSize::new(width, height),
            root_background_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
            renderables: Vec::new(),
        }
    }

    pub fn run(mut self) {
        loop {
            if self.handle_events() {
                break;
            }
            self.update_elements();

            if self.display_list_dirty() {
                self.construct_display_list();
            }

            gl::clear(gl::COLOR_BUFFER_BIT);

            self.renderer.update();
            self.renderer.render(self.size);

            self.window.swap_buffers().ok();
        }
    }

    fn handle_events(&mut self) -> bool {
        for event in self.window.poll_events() {
            // println!("Handling event: {:?}", event);
            for renderable in &self.renderables {
                renderable.borrow_mut().handle_window_event(&event);
            }

            match event {
                glutin::Event::Closed => break,
                glutin::Event::Resized(width, height) => {
                    println!("Resized");
                    self.size = DeviceUintSize::new(width, height);
                }
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => {
                    return true;
                }
                _ => ()
            }
        }
        false
    }

    pub fn add_renderable<T: Renderable + 'static>(&mut self, renderable: T) {
        self.renderables.push(Box::new(RefCell::new(renderable)));
    }

    fn display_list_dirty(&self) -> bool {
        for renderable in &self.renderables {
            if renderable.borrow_mut().is_dirty() {
                return true;
            }
        }
        false
    }

    fn update_elements(&self) {
        for renderable in &self.renderables {
            renderable.borrow_mut().update();
        }
    }

    fn construct_display_list(&self) {
        let mut builder = DisplayListBuilder::new(self.pipeline_id);

        let size = LayoutSize::new(self.size.width as f32, self.size.height as f32);

        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), size);
        let clip_region = {
            let complex = ComplexClipRegion::new(
                LayoutRect::new(LayoutPoint::new(50.0, 50.0), LayoutSize::new(100.0, 100.0)),
                BorderRadius::uniform(20.0));

            builder.new_clip_region(&bounds, vec![complex], None)
        };
        builder.push_stacking_context(ScrollPolicy::Scrollable,
                                      bounds,
                                      clip_region,
                                      0,
                                      &LayoutTransform::identity(),
                                      &LayoutTransform::identity(),
                                      MixBlendMode::Normal,
                                      Vec::new());
        
        for renderable in &self.renderables {
            renderable.borrow().render(&mut builder);
        }

        builder.pop_stacking_context();

        self.api.set_root_display_list(
            Some(self.root_background_color),
            self.epoch,
            size,
            builder);
        self.api.set_root_pipeline(self.pipeline_id);
    }
}
