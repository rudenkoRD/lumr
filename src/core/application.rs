use std::sync::Arc;

use vulkano::{
    sync::{self,future::FenceSignalFuture, FlushError, GpuFuture}, swapchain::{AcquireError, SwapchainPresentInfo, self},
};
use winit::dpi::PhysicalSize;

use crate::graphics::{
    window::GraphicsWindow,
    vulkan_instance::VulkanInstanse,
    device_manager::DeviceManager, swapchain_manager::SwapchainManager, renderer::Renderer,
};

pub struct Application {
    graphics_window: GraphicsWindow,
    device_manager: Arc<DeviceManager>,
    swapchain_manager: SwapchainManager,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Application {
        let vulkan_instanse = VulkanInstanse::new();
        let graphics_window = GraphicsWindow::new(vulkan_instanse.instance.clone());
        let device_manager = DeviceManager::new(&vulkan_instanse.instance, &graphics_window.surface);
        let swapchain_manager = SwapchainManager::new(&graphics_window, device_manager.clone());
        let renderer = Renderer::new(&device_manager, &swapchain_manager, &graphics_window.viewport);

        Application { graphics_window, device_manager, swapchain_manager, renderer }
    }

    pub fn run(mut self) {
        let mut recreate_swapchain = false;
        let frames_in_flight = self.swapchain_manager.images.len();
        let mut fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; frames_in_flight];
        let mut previous_fence_i = 0;

        self.graphics_window.run_event_loop(
            move |updated_dimensions: PhysicalSize<u32>, viewport, window_resized| {
                if recreate_swapchain || window_resized {
                    recreate_swapchain = false;

                    self.swapchain_manager.recreate(updated_dimensions);

                    if window_resized {
                        self.renderer.recreate_command_buffer(&self.device_manager, &self.swapchain_manager, viewport);
                    }
                } 

                let (image_i, suboptimal, acquire_future) =
                match swapchain::acquire_next_image(self.swapchain_manager.swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}"),
                };
        
                if suboptimal {
                    recreate_swapchain = true;
                }
                
                if let Some(image_fence) = &fences[image_i as usize] {
                    image_fence.wait(None).unwrap();
                }

                let previous_future = match fences[previous_fence_i as usize].clone() {
                    None => {
                        let mut now = sync::now(self.device_manager.device.clone());
                        now.cleanup_finished();

                        now.boxed()
                    }
                    Some(fence) => fence.boxed(),
                };

                let future = previous_future
                    .join(acquire_future)
                    .then_execute(
                        self.device_manager.queue.clone(),
                        self.renderer.command_buffers[image_i as usize].clone()
                    )
                    .unwrap()
                    .then_swapchain_present(
                        self.device_manager.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(self.swapchain_manager.swapchain.clone(), image_i),
                    )
                    .then_signal_fence_and_flush();

                fences[image_i as usize] = match future {
                    Ok(value) => Some(Arc::new(value)),
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        None
                    }
                    Err(e) => {
                        println!("failed to flush future: {e}");
                        None
                    }
                };

                previous_fence_i = image_i;
            },
        );
    }
}