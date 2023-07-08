use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::PhysicalSize,
};
use vulkano::{
    instance::Instance,
    swapchain::Surface, pipeline::graphics::viewport::Viewport,
};
use vulkano_win::VkSurfaceBuild;
use std::sync::Arc;

pub struct GraphicsWindow {
    pub window: Arc<Window>,
    pub surface: Arc<Surface>,
    event_loop: EventLoop<()>,
    pub viewport: Viewport,
}

impl GraphicsWindow {
    pub fn new(instance: Arc<Instance>) -> GraphicsWindow {
        let event_loop = EventLoop::new();

        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance)
            .unwrap();

        let window = surface
            .object()
            .unwrap()
            .clone()
            .downcast::<Window>()
            .unwrap();

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: window.inner_size().into(),
            depth_range: 0.0..1.0,
        };

        GraphicsWindow { window, surface, event_loop, viewport }
    }

    pub fn run_event_loop<F>(mut self, mut on_update: F)
    where
        F: 'static + FnMut(PhysicalSize<u32>, &Viewport, bool),
    {
        let mut window_resized = false;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    window_resized = true;
                }
                Event::MainEventsCleared => {
                    if window_resized {
                        self.viewport.dimensions = self.window.inner_size().into();
                    }

                    on_update(self.window.inner_size(), &self.viewport, window_resized);

                    window_resized = false;
                }
                 _ => (),
            }
        });
    }
}

