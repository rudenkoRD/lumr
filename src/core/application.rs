use crate::graphics::{
    window::GraphicsWindow,
    vulkan_instance::VulkanInstanse,
    device_manager::DeviceManager,
};

pub struct Application {
    pub window: GraphicsWindow,
}

impl Application {
    pub fn new() -> Application {
        let vulkan_instanse = VulkanInstanse::new();
        let window = GraphicsWindow::new(vulkan_instanse.instance.clone());
        let _ = DeviceManager::new(&vulkan_instanse.instance, &window.surface);

        Application { window }
    }
}