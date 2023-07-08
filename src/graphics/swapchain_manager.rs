use std::sync::Arc;

use vulkano::{
    swapchain::{Swapchain, SwapchainCreateInfo, SwapchainCreationError}, 
    image::{SwapchainImage, ImageUsage, view::ImageView},
    device::Device, 
    render_pass::{RenderPass, Framebuffer, FramebufferCreateInfo},
};
use winit::dpi::PhysicalSize;
use super::{window::GraphicsWindow, device_manager::DeviceManager};

pub struct SwapchainManager {
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<SwapchainImage>>,
    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
}

impl SwapchainManager {
    pub fn new(graphics_window: &GraphicsWindow, device_manager: Arc<DeviceManager>) -> SwapchainManager {
        let (swapchain, images) = {
            let caps = device_manager.physical_device
                .surface_capabilities(graphics_window.surface.as_ref(), Default::default())
                .expect("failed to get surface capabilities");

            let dimensions = graphics_window.window.inner_size();
            let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
            let image_format = Some(
                device_manager.physical_device
                    .surface_formats(&graphics_window.surface, Default::default())
                    .unwrap()[0]
                    .0,
            );

            Swapchain::new(
                device_manager.device.clone(),
                graphics_window.surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: caps.min_image_count,
                    image_format,
                    image_extent: dimensions.into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha,
                    ..Default::default()
                },
            )
            .unwrap()
        };

        let render_pass = Self::get_render_pass(device_manager.device.clone(), swapchain.clone());
        let framebuffers = Self::get_framebuffers(&images, render_pass.clone());

        SwapchainManager { swapchain, images, render_pass, framebuffers }
    }

    pub fn recreate(&mut self, updated_dimensions: PhysicalSize<u32>) {
        let (new_swapchain, new_images) = match self.swapchain.recreate(
            SwapchainCreateInfo {
            image_extent: updated_dimensions.into(),
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("failed to recreate swapchain: {e}"),
        };
        self.swapchain = new_swapchain;
        self.framebuffers = Self::get_framebuffers(&new_images, self.render_pass.clone());
    }

    fn get_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(), // set the format the same as the swapchain
                    samples: 1,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap()
    }

    fn get_framebuffers(
        images: &[Arc<SwapchainImage>],
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }
}
