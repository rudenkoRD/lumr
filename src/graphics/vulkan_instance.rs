use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    instance:: {Instance, InstanceCreateInfo},
};

pub struct VulkanInstanse {
    pub instance: Arc<Instance>,
}

impl VulkanInstanse {
    pub fn new() -> VulkanInstanse {
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let required_extensions = vulkano_win::required_extensions(&library);
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        VulkanInstanse { instance }
    }
}
