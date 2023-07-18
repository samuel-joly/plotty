mod demos;
mod scene;
use demos::rotating_cube;
use scene::Scene;

use std::num::NonZeroU32;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
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

    let mut scene: Scene = Scene {
        camera: (0.0, 0.0, 0.0),
        screen: (0.0, 0.0, 1.0),
        width,
        height,
    };
    let mut frame: f64 = 0.001;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x000000);

                for faces in rotating_cube((-1.0, -1.0, 2.0), 0.25, frame) {
                    for tr in faces {
                        let triangle = scene.project(tr);
                        scene.draw_triangle(triangle, &mut buffer);
                    }
                }
                buffer.present().unwrap();
                frame += 0.01;
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
