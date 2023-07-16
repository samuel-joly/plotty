use std::f64::consts::PI;
use std::num::NonZeroU32;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn cube(
    eye: &Vec3d,
    screen: &Vec3d,
    width: u32,
    height: u32,
    mut_triangle: &mut Vec<Vec<(f64, f64)>>,
    frame: f64,
) {

    let mut y = -0.25;
    let radius: f64 = 0.5;
    let fr_sq = frame + (PI / 2.0);
    let fr_sq3 = frame + (PI * 1.5);
    *mut_triangle = vec![];
    let cube = vec![
        vec![vec![
            (frame.cos() * radius, radius, 2.0 + frame.sin() * radius),
            (fr_sq.cos() * radius, radius, 2.0 + fr_sq.sin() * radius),
            (-frame.cos() * radius, radius, 2.0 + -frame.sin() * radius),
        ]],
        vec![vec![
            (frame.cos() * radius, y, 2.0 + frame.sin() * radius),
            (fr_sq.cos() * radius, y, 2.0 + fr_sq.sin() * radius),
            (-frame.cos() * radius, y, 2.0 + -frame.sin() * radius),
        ]],
        vec![
            vec![
                (frame.cos() * radius, y, 2.0 + frame.sin() * radius),
                (frame.cos() * radius, radius, 2.0 + frame.sin() * radius),
                (fr_sq.cos() * radius, radius, fr_sq.sin() * radius + 2.0),
            ],
            vec![
                (frame.cos() * radius, y, 2.0 + frame.sin() * radius),
                (fr_sq.cos() * radius, y, 2.0 + fr_sq.sin() * radius),
                (fr_sq.cos() * radius, radius, fr_sq.sin() * radius + 2.0),
            ],
        ],
        vec![
            vec![
                (-frame.cos() * radius, y, 2.0 + -frame.sin() * radius),
                (-frame.cos() * radius, radius, 2.0 + -frame.sin() * radius),
                (-fr_sq.cos() * radius, radius, -fr_sq.sin() * radius + 2.0),
            ],
            vec![
                (-frame.cos() * radius, y, 2.0 + -frame.sin() * radius),
                (-fr_sq.cos() * radius, y, 2.0 + -fr_sq.sin() * radius),
                (-fr_sq.cos() * radius, radius, -fr_sq.sin() * radius + 2.0),
            ],
        ],
        vec![
            vec![
                (frame.cos() * radius, y, 2.0 + frame.sin() * radius),
                (frame.cos() * radius, radius, 2.0 + frame.sin() * radius),
                (fr_sq3.cos() * radius, radius, fr_sq3.sin() * radius + 2.0),
            ],
            vec![
                (frame.cos() * radius, y, 2.0 + frame.sin() * radius),
                (fr_sq3.cos() * radius, y, 2.0 + fr_sq3.sin() * radius),
                (fr_sq3.cos() * radius, radius, fr_sq3.sin() * radius + 2.0),
            ],
        ],
        vec![
            vec![
                (-frame.cos() * radius, y, 2.0 + -frame.sin() * radius),
                (-frame.cos() * radius, radius, 2.0 + -frame.sin() * radius),
                (-fr_sq3.cos() * radius, radius, -fr_sq3.sin() * radius + 2.0),
            ],
            vec![
                (-frame.cos() * radius, y, 2.0 + -frame.sin() * radius),
                (-fr_sq3.cos() * radius, y, 2.0 + -fr_sq3.sin() * radius),
                (-fr_sq3.cos() * radius, radius, -fr_sq3.sin() * radius + 2.0),
            ],
        ],
    ];

    let mut triangle_points: Vec<(f64, f64)> = vec![];
    for faces in &cube {
        for triangle in faces {
            for point in triangle {
                let x = point.0 / point.2;
                let y = point.1 / point.2;
                let scale_x = ((x + 1.0) / 2.0) * width as f64;
                let scale_y = (1.0 - (y + 1.0) / 2.0) * height as f64;
                triangle_points.push((scale_x, scale_y));
            }
            mut_triangle.push(triangle_points);
            triangle_points = vec![];
        }
    }
}

#[derive(Debug)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}
fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("plotty")
        .with_inner_size(LogicalSize::new(600, 600));
    let window = window_builder.build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let eye = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let screen = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    let mut cube_triangle = vec![];
    let mut width = 0;
    let mut height = 0;
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
                        NonZeroU32::new(width).unwrap(),  // 1882 | /2 = 941
                        NonZeroU32::new(height).unwrap(), // 1006 | /2 = 503
                    )
                    .unwrap();
                cube(&eye, &screen, width, height, &mut cube_triangle, frame);
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x000000);
                cube(&eye, &screen, width, height, &mut cube_triangle, frame);
                let colors = vec![0xFF0000, 0x00FF00, 0x0000FF];

                for triangle in &cube_triangle {
                    let dist_x_ab: f64 = triangle[1].0 - triangle[0].0;
                    let dist_y_ab: f64 = triangle[1].1 - triangle[0].1;

                    let dist_x_ac: f64 = triangle[2].0 - triangle[0].0;
                    let dist_y_ac: f64 = triangle[2].1 - triangle[0].1;

                    let dist_x_bc: f64 = triangle[1].0 - triangle[2].0;
                    let dist_y_bc: f64 = triangle[1].1 - triangle[2].1;

                    let dists = vec![
                        (dist_x_ab, dist_y_ab, triangle[0]),
                        (dist_x_bc, dist_y_bc, triangle[2]),
                        (dist_x_ac, dist_y_ac, triangle[0]),
                    ];

                    for (cnt, (dist_x, dist_y, triangle)) in dists.into_iter().enumerate() {
                        let total_dist = dist_x.abs() + dist_y.abs();
                        for j in 0..total_dist.floor() as i32 {
                            let x = triangle.0 + dist_x * j as f64 / total_dist;
                            let y = triangle.1 + dist_y * j as f64 / total_dist;
                            let index = x.floor() as i32 + (y.floor() as i32 * width as i32);
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
