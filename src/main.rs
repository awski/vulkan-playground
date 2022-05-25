use vulkano::{
    device::physical::PhysicalDevice,
    instance::{Instance, InstanceCreateInfo},
};
use vulkano_win::VkSurfaceBuild;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let required_extensions = vulkano_win::required_extensions();

    let instance = Instance::new(InstanceCreateInfo {
        enabled_extensions: required_extensions,
        ..Default::default()
    })
    .unwrap();

    let device = PhysicalDevice::enumerate(&instance).next().unwrap();
    let event = EventLoop::new();

    let surface = WindowBuilder::new()
        .build_vk_surface(&event, instance)
        .unwrap();

    
}
