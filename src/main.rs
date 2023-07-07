use std::num::NonZeroU32;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn setup() -> Vec<Vec3d> {
    vec![
        Vec3d {
            x: 1.0,
            y: 1.0,
            z: 3.0,
        },
        Vec3d {
            x: -1.0,
            y: 1.0,
            z: 3.0,
        },
        Vec3d {
            x: 1.0,
            y: -1.0,
            z: 3.0,
        },
        Vec3d {
            x: -1.0,
            y: -1.0,
            z: 3.0,
        },
        Vec3d {
            x: 1.0,
            y: 1.0,
            z: 4.0,
        },
        Vec3d {
            x: -1.0,
            y: -1.0,
            z: 4.0,
        },
        Vec3d {
            x: 1.0,
            y: -1.0,
            z: 4.0,
        },
        Vec3d {
            x: -1.0,
            y: 1.0,
            z: 4.0,
        },
    ]
}

#[derive(Debug)]
struct Vec2d {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}
fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        //.with_inner_size(LogicalSize::new(600, 600))
        .with_title("plotty");
    let window = window_builder.build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let mut frame: f32 = 0.0;
    let mut eye = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    let mut screen = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 2.0,
    };

    let mut width = 0;
    let mut height = 0;

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
                        NonZeroU32::new(width).unwrap(),  // 1882 | /2 = 941
                        NonZeroU32::new(height).unwrap(), // 1006 | /2 = 503
                    )
                    .unwrap();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x000000);
                screen.z = 0.8 * frame.sin() + 2.0;
                //eye.x += 0.1 * frame.sin();
                eye.x = frame.cos();
                let mut faces: Vec<(f32, f32)> = vec![];
                for vertex in setup() {
                    let computed_x =
                        ((vertex.x - eye.x) * (screen.z - eye.z) / (vertex.z - eye.z)) + eye.x;
                    let computed_y =
                        ((vertex.y - eye.y) * (screen.z - eye.z) / (vertex.z - eye.z)) + eye.y;
                    let v2d = Vec2d {
                        x: ((computed_x + 1.0) / 2.0) * (width * height / width) as f32,
                        y: (1.0 - (computed_y + 1.0) / 2.0) * height as f32,
                    };
                    faces.push((v2d.x, v2d.y));
                    for point in &faces {
                        for point2 in &faces {
                            let mut dist_y: f32 = 0.0;
                            let mut dist_x: f32 = 0.0;
                                dist_y = point2.1 - point.1;
                                dist_x = point2.0 - point.0;
                            for i in 0..(dist_x + dist_y).floor() as i32 {
                                let x = point.0 + dist_x * i as f32 / (dist_x + dist_y);
                                let y = point.1 + dist_y * i as f32 / (dist_x + dist_y);
                                let index = x.floor() as i32
                                    + (y.floor() as i32 * width as i32)
                                    + (width / 4) as i32;
                                if buffer.get(index as usize).is_some() {
                                    buffer[index as usize] = 0xFFFFFF;
                                }
                            }
                        }
                    }
                    let index = v2d.x.floor() as i32
                        + (v2d.y.floor() as i32 * width as i32)
                        + (width / 4) as i32;
                    if buffer.get(index as usize).is_some() {
                        buffer[index as usize] = 0xFFFFFF;
                    }
                }

                //                for i in 1..lines.len() {
                //                    let pix_x = lines[i].x - lines[i-1].x;
                //                    let pix_y = lines[i].y - lines[i-1].y;
                //                    let pix = (pix_x + pix_y).floor() as u32;
                //                    for j in 0..pix
                //                    {
                //                        let new_x = lines[i-1].x+ pix_x*j as f32 /pix as f32;
                //                        let new_y = lines[i-1].y+ pix_y*j as f32 /pix as f32;
                //                        let index = new_x.floor() as i32+ (new_y.floor() as i32* width as i32) + (width/4) as i32;
                //                        if buffer.get(index as usize).is_some() {
                //                            buffer[index as usize] = 0xFFFFFF;
                //                        }
                //                    }
                //                }
                frame += 0.01;
                buffer.present().unwrap();
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
