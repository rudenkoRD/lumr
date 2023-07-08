use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use vulkano::{
    instance::Instance,
    swapchain::Surface,
};
use vulkano_win::VkSurfaceBuild;
use std::sync::Arc;

pub struct GraphicsWindow {
    window: Arc<Window>,
    pub surface: Arc<Surface>,
    event_loop: EventLoop<()>,
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

        GraphicsWindow { window, surface, event_loop }
    }

    pub fn run_event_loop(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}

