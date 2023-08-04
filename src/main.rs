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
    let radius: f64 = 1.0;
    let mut traj:Vec<u32> = vec![];
    let mut traj_count = 1;

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
                for i in 0..1000 {
                    let mut x = (i as f64).cos() * radius;
                    let mut y = (i as f64).sin() * radius;
                    (x, y) = scene.project(vec![(x, y, 3.0)], false)[0];
                    let index = x.floor() as i32 + (y.floor() as i32 * width as i32);
                    buffer[index as usize] = 0xFFFFFF;
                }
                for j in 0..400 {
                    let mut x = ((j as f64).cos() * radius / 5.0) + (frame).cos() * radius;
                    let mut y = ((j as f64).sin() * radius / 5.0) + (frame).sin() * radius;
                    (x, y) = scene.project(vec![(x, y, 3.0)], false)[0];
                    let index = x.floor() as i32 + (y.floor() as i32 * width as i32);
                    buffer[index as usize] = 0xFF0000;
                }
                let mut pt = false;
                for k in 0..20 {
                    let mut x = ((k as f64).cos() * radius / 50.0)
                        + (frame*5.0).cos() * radius / 5.0
                        + (frame).cos() * radius;
                    let mut y = ((k as f64).sin() * radius / 50.0)
                        + (frame*5.0).sin() * radius / 5.0
                        + (frame).sin() * radius;
                    (x, y) = scene.project(vec![(x, y, 3.0)], false)[0];
                    let index = x.floor() as i32 + (y.floor() as i32 * width as i32);
                    if buffer.get(index as usize).is_some() {
                        buffer[index as usize] = 0x00FF00;
                        if pt == false {
                            if traj.len() <= 100 {
                                traj.push(index as u32);
                            } else {
                                traj[traj_count] = index as u32;
                                traj_count += 1;
                            }
                            pt = true;
                        }
                    }
                }
                for i in &traj {
                    buffer[*i as usize] = 0xFFFF00;
                }
                if traj_count == 100 {
                    traj_count = 0;
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
