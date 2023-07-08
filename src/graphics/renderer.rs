use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo,
    SubpassContents,
};
use vulkano::device::{Device, Queue};
use vulkano::memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryUsage};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::render_pass::{RenderPass, Subpass, Framebuffer};
use vulkano::shader::ShaderModule;
use std::sync::Arc;

use super::device_manager::DeviceManager;
use super::shaders;
use super::swapchain_manager::SwapchainManager;

pub struct Renderer {
    pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    vertex_buffer: Subbuffer<[MyVertex]>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

impl Renderer {
    pub fn new(device_manager: &Arc<DeviceManager>, swapchain_manager: &SwapchainManager, viewport: &Viewport) -> Renderer {
        let memory_allocator = StandardMemoryAllocator::new_default(device_manager.device.clone());

        let vertex1 = MyVertex {
            position: [-0.5, -0.5],
        };
        let vertex2 = MyVertex {
            position: [0.0, 0.5],
        };
        let vertex3 = MyVertex {
            position: [0.5, -0.25],
        };
        let vertex_buffer = Buffer::from_iter(
            &memory_allocator,
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            vec![vertex1, vertex2, vertex3].into_iter(),
        )
        .unwrap();

        let vs = shaders::vertex::vertex_shader::load(device_manager.device.clone()).expect("failed to create shader module");
        let fs = shaders::fragment::fragment_shader::load(device_manager.device.clone()).expect("failed to create shader module");

        let pipeline = Self::get_pipeline(
            device_manager.device.clone(),
            vs.clone(),
            fs.clone(),
            swapchain_manager.render_pass.clone(),
            viewport.clone(),
        );

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device_manager.device.clone(), Default::default());

        let command_buffers = Self::get_command_buffers(
            &command_buffer_allocator,
            &device_manager.queue,
            &pipeline,
            &swapchain_manager.framebuffers,
            &vertex_buffer,
        );

        Renderer { command_buffers, vertex_buffer, command_buffer_allocator, vs, fs }
    }

    pub fn recreate_command_buffer(&mut self, device_manager: &Arc<DeviceManager>, swapchain_manager: &SwapchainManager, viewport: &Viewport) {
        let new_pipeline = Self::get_pipeline(
            device_manager.device.clone(),
            self.vs.clone(),
            self.fs.clone(),
            swapchain_manager.render_pass.clone(),
            viewport.clone(),
        );

        self.command_buffers = Self::get_command_buffers(
            &self.command_buffer_allocator,
            &device_manager.queue,
            &new_pipeline,
            &swapchain_manager.framebuffers,
            &self.vertex_buffer,
        );  
    }

    fn get_pipeline(
        device: Arc<Device>,
        vs: Arc<ShaderModule>,
        fs: Arc<ShaderModule>,
        render_pass: Arc<RenderPass>,
        viewport: Viewport,
    ) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            .vertex_input_state(MyVertex::per_vertex())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .render_pass(Subpass::from(render_pass, 0).unwrap())
            .build(device)
            .unwrap()
    }
    
    fn get_command_buffers(
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: &Arc<Queue>,
        pipeline: &Arc<GraphicsPipeline>,
        framebuffers: &[Arc<Framebuffer>],
        vertex_buffer: &Subbuffer<[MyVertex]>,
    ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
        framebuffers
            .iter()
            .map(|framebuffer| {
                let mut builder = AutoCommandBufferBuilder::primary(
                    command_buffer_allocator,
                    queue.queue_family_index(),
                    CommandBufferUsage::MultipleSubmit,
                )
                .unwrap();
    
                builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                            ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                        },
                        SubpassContents::Inline,
                    )
                    .unwrap()
                    .bind_pipeline_graphics(pipeline.clone())
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass()
                    .unwrap();
    
                Arc::new(builder.build().unwrap())
            })
            .collect()
    }
}
