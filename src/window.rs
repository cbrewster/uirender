use euclid::{Size2D, Point2D, Rect, Matrix4D};
use gleam::gl;
use glutin;
use primitives::Renderable;
use std::ffi::CStr;
use webrender::RendererOptions;
use webrender::renderer::Renderer;
use webrender_traits::{AuxiliaryListsBuilder, ColorF, DisplayListBuilder};
use webrender_traits::{Epoch, MixBlendMode, PipelineId, RendererKind, RenderApi};
use webrender_traits::{RenderNotifier, ScrollPolicy, StackingContext};

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
                         _: Option<Size2D<f32>>) {
    }
}

pub struct Window {
    window: glutin::Window,
    renderer: Renderer,
    api: RenderApi,
    pipeline_id: PipelineId,
    epoch: Epoch,
    size: Size2D<u32>,
    root_background_color: ColorF,
    renderables: Vec<Box<Renderable>>,
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
            gl::clear_color(0.3, 0.0, 0.0, 1.0);
        }

        let version = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
            String::from_utf8(data).unwrap()
        };

        println!("OpenGl version {}", version);

        let (width, height) = window.get_inner_size().unwrap();

        let opts = RendererOptions {
            device_pixel_ratio: 1.0,
            resource_override_path: None,
            enable_aa: false,
            enable_msaa: false,
            enable_profiler: false,
            enable_recording: false,
            enable_scrollbars: false,
            debug: true,
            precache_shaders: true,
            renderer_kind: RendererKind::Native,
            enable_subpixel_aa: false,
        };

        let (mut renderer, sender) = Renderer::new(opts);
        let api = sender.create_api();

        let notifier = Box::new(Notifier::new(window.create_window_proxy()));
        renderer.set_render_notifier(notifier);

        Window {
            window: window,
            renderer: renderer,
            api: api,
            pipeline_id: PipelineId(0, 0),
            epoch: Epoch(0),
            size: Size2D::new(width, height),
            root_background_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
            renderables: Vec::new(),
        }
    }

    pub fn run(mut self) {
        for event in self.window.wait_events() {
            gl::clear(gl::COLOR_BUFFER_BIT);
            self.construct_display_list();

            self.renderer.update();
            self.renderer.render(self.size);

            self.window.swap_buffers().ok();

            match event {
                glutin::Event::Closed => break,
                glutin::Event::KeyboardInput(_element_state, scan_code, _virtual_key_code) => {
                    if scan_code == 9 {
                        break;
                    }
                }
                _ => ()
            }
        }
    }

    pub fn add_renderable<T: Renderable + 'static>(&mut self, renderable: T) {
        self.renderables.push(Box::new(renderable));
    }

    fn construct_display_list(&self) {
        let mut auxiliary_lists_builder = AuxiliaryListsBuilder::new();
        let mut builder = DisplayListBuilder::new();

        let size = Size2D::new(self.size.width as f32, self.size.height as f32);

        let bounds = Rect::new(Point2D::new(0.0, 0.0), size);
        let context = StackingContext::new(ScrollPolicy::Scrollable,
                                           bounds,
                                           bounds,
                                           0,
                                           &Matrix4D::identity(),
                                           &Matrix4D::identity(),
                                           MixBlendMode::Normal,
                                           Vec::new(),
                                           &mut auxiliary_lists_builder);
        builder.push_stacking_context(context);

        for renderable in &self.renderables {
            renderable.render(&mut builder);
        }

        builder.pop_stacking_context();

        self.api.set_root_display_list(
            self.root_background_color,
            self.epoch,
            self.pipeline_id,
            size,
            builder.finalize(),
            auxiliary_lists_builder.finalize());
        self.api.set_root_pipeline(self.pipeline_id);
    }
}
