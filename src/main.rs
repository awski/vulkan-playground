use std::vec;

use vulkano::{
    device::{physical::{PhysicalDevice, PhysicalDeviceType}, DeviceExtensions, Device, self, DeviceCreateInfo, QueueCreateInfo},
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

    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    let event = EventLoop::new();

    let surface = WindowBuilder::new()
        .build_vk_surface(&event, instance.clone())
        .unwrap();

    let (phy_device, queue_family) = PhysicalDevice::enumerate(&instance)
        .filter(|&p| p.supported_extensions().is_superset_of(&device_ext))
        .filter_map(|p| {
            p.queue_families()
                .find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap())
                .map(|q| (p, q))
        }).min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
        }).unwrap();

    let (device, mut queues) = Device::new(
        phy_device,
        DeviceCreateInfo {
            enabled_extensions: phy_device
                .required_extensions()
                .union(&device_ext),
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    ).unwrap();
}