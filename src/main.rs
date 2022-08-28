use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage},
    device::{physical::PhysicalDevice, Device, DeviceCreateInfo, QueueCreateInfo},
    instance::{Instance, InstanceCreateInfo},
    sync::{self, GpuFuture},
};

fn main() {
    // Create an instance
    let instance =
        Instance::new(InstanceCreateInfo::default()).expect("failed to create the instance");

    // Get phisical devices
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    // Show family queues
    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    // Create a device
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    // Create buffer
    let _data: i32 = 12;
    let _buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, _data)
        .expect("failed to create buffer");

    let mut _content = _buffer.write().unwrap();

    // Example: copy from source buffer to destination
    let source_content: Vec<i32> = (0..64).collect();
    let source =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
            .expect("failed to create source buffer");

    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        destination_content,
    )
    .expect("failed to create destination buffer");

    // Create command Buffer
    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .copy_buffer(source.clone(), destination.clone())
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let src_content = source.read().unwrap();

    let dst_content = destination.read().unwrap();

    assert_eq!(&*src_content, &*dst_content);
}
