use std::{f64::consts::PI, num::NonZeroU32};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let mut tick: u32 = 0;
    let tick_increment: u32 = 5;

    let def_r = 400.0;
    let pla_r = 20.0;

    let deferant = Circle::new(def_r, 0xFFFFFF, false);
    let planet = Circle::new(pla_r, 0xFF0000, true);
    let max_planet_epicycle = Circle::new(pla_r + def_r, 0x0000FF, false);

    let drawable: Vec<(u32, Vec<(f64, f64)>)> = vec![(planet.color, planet.pixels)];
    let backgrounds: Vec<(u32, Vec<(f64, f64)>)> = vec![
        (deferant.color, deferant.pixels),
        (max_planet_epicycle.color, max_planet_epicycle.pixels),
    ];
    let mut width = 0;
    let mut _height = 0;
    let mut bg: Vec<u32> = vec![];
    let mut drawed: Vec<usize> = vec![];
    let mut mid_grid: u32 = 0;
    let mut frame_time: Vec<u128> = vec![];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                (width, _height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                if width != 0 {
                    if tick == 0 {
                        surface
                            .resize(
                                NonZeroU32::new(width).unwrap(),   // 1882 | /2 = 941
                                NonZeroU32::new(_height).unwrap(), // 1006 | /2 = 503
                            )
                            .unwrap();
                        let mut buffer = surface.buffer_mut().unwrap();
                        for index in 0..(width * _height) {
                            buffer[index as usize] = 0x000000;
                        }
                        for (color, pixels) in &backgrounds {
                            for (x, y) in pixels {
                                let index = x.floor() as i32
                                    + 941
                                    + ((y.floor() as i32) * width as i32)
                                    + ((width * _height) / 2) as i32;
                                buffer[index as usize] = *color;
                            }
                        }
                        bg = buffer.to_vec();
                        mid_grid = ((width * _height) / 2) - (width / 2);
                    }
                }
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = std::time::Instant::now();
                let mut buffer = surface.buffer_mut().unwrap();
                for index in &drawed {
                    buffer[*index] = bg[*index];
                }
                drawed = vec![];
                let tick_cos = ((tick as f64 / 1000.0).cos() * deferant.radius).floor();
                let tick_sin = ((tick as f64 / 1000.0).sin() * deferant.radius).floor();
                for (color, pixels) in &drawable {
                    for (x, y) in pixels {
                        let index = x.floor()
                            + tick_cos
                            + (y.floor() + tick_sin) * width as f64
                            + mid_grid as f64;
                        drawed.push(index.floor() as usize);
                        buffer[index.floor() as usize] = *color;
                    }
                }
                //std::thread::sleep(std::time::Duration::from_millis(5));
                buffer.present().unwrap();
                tick += tick_increment;
                window.request_redraw();
                frame_time.push(now.elapsed().as_millis());
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
                let mut sum = 0;
                for i in &frame_time {
                    sum += i;
                }
                let avg = sum as f64 / frame_time.len() as f64;
                dbg!(avg);
            }
            _ => {}
        }
    });
}

#[derive(Debug)]
struct Circle {
    pixels: Vec<(f64, f64)>,
    color: u32,
    _perimeter: f64,
    radius: f64,
}

impl Circle {
    fn new(r: f64, c: u32, fill: bool) -> Circle {
        let mut pixels: Vec<(f64, f64)> = vec![];
        for i in 0..((2.0 * (PI * r)) * 2.5).floor() as i32 {
            pixels.push(((i as f64 / 2.5).cos() * r, (i as f64 / 2.5).sin() * r));
        }
        if fill {
            let mut fill_pix: Vec<Vec<(f64, f64)>> = vec![];
            for i in 0..5 {
                let step = i as f64 / 2.0;
                fill_pix.push(Circle::new(step, c, false).pixels);
            }
            let filled_pix = fill_pix.concat();
            pixels = vec![pixels.clone(), filled_pix].concat();
        }
        let deferant = Circle {
            pixels,
            color: c,
            _perimeter: (r * 2.0) * PI,
            radius: r,
        };
        deferant
    }

    pub fn _check_proportion(&self) {
        let mut data: Vec<f64> = vec![];
        for (i, j) in &self.pixels {
            data.push(i.powf(2.0) + j.powf(2.0));
        }
        let mut sum: f64 = 0.0;
        for i in &data {
            sum += i / 100 as f64;
        }
        dbg!(sum / data.len() as f64);
    }
}
