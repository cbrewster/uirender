use gleam::gl;
use glutin;
use object::Object;
use webrender::RendererOptions;
use webrender::renderer::Renderer;
use webrender_traits::{ColorF, ClipRegion, DisplayListBuilder};
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
    epoch: Epoch,
    size: DeviceUintSize,
    root_background_color: ColorF,
    pipeline_id: PipelineId,
    objects: Vec<Object>,
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

        let pipeline_id = PipelineId(0, 0);
        api.set_root_pipeline(pipeline_id);

        Window {
            window: window,
            renderer: renderer,
            api: api,
            epoch: Epoch(0),
            size: DeviceUintSize::new(width, height),
            root_background_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
            pipeline_id: pipeline_id,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn run(mut self) {
        loop {
            if self.handle_events() {
                break;
            }

            self.construct_display_list();

            self.renderer.update();
            self.renderer.render(self.size);

            self.window.swap_buffers().ok();
        }
    }

    fn handle_events(&mut self) -> bool {
        for event in self.window.poll_events() {
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

    fn construct_display_list(&mut self) {
        let mut builder = DisplayListBuilder::new(self.pipeline_id);

        let size = LayoutSize::new(self.size.width as f32, self.size.height as f32);

        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), size);

        builder.push_stacking_context(ScrollPolicy::Scrollable,
                                      bounds,
                                      ClipRegion::simple(&bounds),
                                      0,
                                      &LayoutTransform::identity(),
                                      &LayoutTransform::identity(),
                                      MixBlendMode::Normal,
                                      Vec::new());

        for object in &self.objects {
            object.build(&mut builder);
        }

        builder.pop_stacking_context();

        let epoch = self.next_epoch();

        self.api.set_root_display_list(
            Some(self.root_background_color),
            epoch,
            size,
            builder);
    }

    fn next_epoch(&mut self) -> Epoch {
        let epoch = self.epoch;
        self.epoch = Epoch(self.epoch.0 + 1);
        epoch
    }
}
