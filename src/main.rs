use std::num::NonZeroU32;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn cube(eye: &Vec3d, screen: &Vec3d, width: u32, height: u32, mut_triangle:&mut Vec<Vec<(f32,f32)>>) {

    *mut_triangle = vec![];
    let cube = vec![
        vec![
            vec![(-1.0, -1.0, 3.0), (-1.0, 1.0, 3.0), (1.0, 1.0, 3.0)],
            vec![(-1.0, -1.0, 3.0), (1.0, -1.0, 3.0), (1.0, 1.0, 3.0)],
        ],
        vec![
            vec![(-1.0, -1.0, 4.0), (-1.0, 1.0, 4.0), (1.0, 1.0, 4.0)],
            vec![(-1.0, -1.0, 4.0), (1.0, -1.0, 4.0), (1.0, 1.0, 4.0)],
        ],
        vec![
            vec![(1.0, 1.0, 3.0), (1.0, 1.0, 4.0), (1.0, -1.0, 3.0)],
            vec![(1.0, -1.0, 3.0), (1.0, -1.0, 4.0), (1.0, 1.0, 4.0)],
        ],
        vec![
            vec![(-1.0, 1.0, 3.0), (-1.0, 1.0, 4.0), (-1.0, -1.0, 3.0)],
            vec![(-1.0, -1.0, 3.0), (-1.0, -1.0, 4.0), (-1.0, 1.0, 4.0)],
        ],
        vec![
            vec![(1.0, 1.0, 3.0), (1.0, 1.0, 4.0), (-1.0, 1.0, 3.0)],
            vec![(-1.0, 1.0, 3.0), (-1.0, 1.0, 4.0), (1.0, 1.0, 4.0)],
        ],
        vec![
            vec![(-1.0, -1.0, 3.0), (-1.0, -1.0, 4.0), (1.0, -1.0, 3.0)],
            vec![(1.0, -1.0, 3.0), (1.0, -1.0, 4.0), (-1.0, -1.0, 3.0)],
        ],
    ];

    let mut triangle_points: Vec<(f32, f32)> = vec![];
    for faces in &cube {
        for triangle in faces {
            for point in triangle {
                let x = ((point.0 - eye.x) * (screen.z - eye.z) / (point.2 - eye.z)) + eye.x;
                let y = ((point.1 - eye.y) * (screen.z - eye.z) / (point.2 - eye.z)) + eye.y;
                let scale_x = ((x+1.0)/2.0)*(width*height/width) as f32;
                let scale_y = (1.0-(y+1.0)/2.0)*height as f32;
                triangle_points.push((scale_x, scale_y));
            }
            mut_triangle.push(triangle_points);
            triangle_points = vec![];
        }
    }
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
        .with_title("plotty");
    let window = window_builder.build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let mut eye = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut screen = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    let mut cube_triangle = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut frame: f32 = 0.0;

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
                cube(&eye, &screen, width, height, &mut cube_triangle);
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x000000);
                //screen.z = 0.8 * frame.sin() + 2.0;
                eye.y = frame.cos();
                eye.x = frame.sin();
                cube(&eye, &screen, width, height, &mut cube_triangle);
                let colors = vec![0xFF0000,  0x00FF00, 0x0000FF];

                for triangle in &cube_triangle {
                    let dist_x_ab: f32 = triangle[1].0 - triangle[0].0;
                    let dist_y_ab: f32 = triangle[1].1 - triangle[0].1;

                    let dist_x_ac: f32 = triangle[2].0 - triangle[0].0;
                    let dist_y_ac: f32 = triangle[2].1 - triangle[0].1;

                    let dist_x_bc: f32 = triangle[1].0 - triangle[2].0;
                    let dist_y_bc: f32 = triangle[1].1 - triangle[2].1;

                    let dists = vec![
                        (dist_x_ab, dist_y_ab, triangle[0]),
                        (dist_x_bc, dist_y_bc, triangle[2]),
                        (dist_x_ac, dist_y_ac, triangle[0]),
                    ];

                    for (cnt, (dist_x, dist_y, triangle)) in dists.into_iter().enumerate() {
                        let total_dist = dist_x.abs() + dist_y.abs();
                        for j in 0..total_dist.floor() as i32 {
                            let x = triangle.0+ dist_x * j as f32 / total_dist;
                            let y = triangle.1+ dist_y * j as f32 / total_dist;
                            let index = x.floor() as i32
                                + (y.floor() as i32 * width as i32)
                                + (width/4) as i32;
                            if buffer.get(index as usize).is_some() {
                                buffer[index as usize] = colors[cnt as usize];
                            }
                        }
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
