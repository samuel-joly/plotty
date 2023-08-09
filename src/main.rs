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
    let mut scene: Scene = Scene::new(5.0, (0.0, 0.0, 0.0), (0.0, 0.0, 5.0), width, height);

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

                //draw_epicycle(&mut buffer, &scene, frame.counter as f64 / 50.0);
                scene.draw_circle(0.0, 1.0+(frame.counter as f64/50.0).cos()*2.0, 8.5, 1.0, 0xFF0000, &mut buffer);

                //scene.draw_text("coucou", 0.0, 0.0, &mut buffer);
                for face in
                    demos::rotating_cube((0.0, (frame.counter as f64/50.0).cos()*2.0, 8.5), 1.0, frame.counter as f64 / 100.0)
                {
                    for tr in face {
                        let triangle = scene.project(tr, true);
                        scene.draw_triangle(triangle, &mut buffer);
                    }
                }
                frame.speed_info();
                buffer.present().unwrap();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {}

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
                // frame.speed_info();
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
