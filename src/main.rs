mod demos;
mod epicycle;
mod frame;
mod scene;
use epicycle::draw_epicycle;
use frame::Frame;
use scene::Scene;
use std::num::NonZeroU32;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let mut width = 600;
    let mut height = 600;
    let window_builder = WindowBuilder::new()
        .with_title("plotty")
        .with_inner_size(LogicalSize::new(width, height));
    let window = window_builder.build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let mut frame = Frame::new(15);
    let mut scene: Scene = Scene::new(
        100.0,
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 100.0),
        width,
        height,
        18.0,
    );

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                scene.width = width;
                scene.height = height;
            }

            Event::MainEventsCleared => {
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x000000);
                frame.update();
                frame.counter += 1;
                demos::draw_rotating_cube(
                    (-50.0, 40.0, 101.0),
                    10.0,
                    &mut buffer,
                    &scene,
                    frame.counter,
                );

                draw_epicycle(
                    (50.0, 50.0, 101.0),
                    20.5,
                    &mut buffer,
                    &scene,
                    frame.counter as f64 / 50.0,
                );
                frame.speed_info(&mut buffer, &scene);
                buffer.present().unwrap();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {}

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::DeviceEvent {
                event: DeviceEvent::Key(key),
                ..
            } => match key.virtual_keycode {
                Some(winit::event::VirtualKeyCode::H) => {}
                _ => {}
            },
            _ => {}
        }
    });
}
