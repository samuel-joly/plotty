use std::{
    collections::HashMap,
    io::Write,
    thread,
    time::{self, Instant},
};

use softbuffer::Buffer;

use crate::scene::Scene;

pub struct Frame {
    pub counter: u32,
    pub timer: Instant,
    pub delta_time: u32,
    pub attributes: Vec<u32>,
    pub times: Vec<f64>,
}

impl Frame {
    pub fn new(delta_time: u32) -> Self {
        let mut fr = Self {
            counter: 0,
            timer: Instant::now(),
            delta_time,
            attributes: vec![],
            times: vec![],
        };
        // Last update time
        fr.attributes.push(0);
        // Avg fps
        fr.attributes.push(0);
        fr
    }

    pub fn update(&mut self) {
        let duration_since_last_call = self.timer.elapsed().as_millis() as u32 - self.attributes[0];
        self.attributes[0] = self.timer.elapsed().as_millis() as u32;
        if self.counter % 100 == 0 {
            self.times.push(self.attributes[1] as f64 / 100.0);
            self.attributes[1] = 0;
        } else {
            self.attributes[1] += duration_since_last_call;
        }
        if self.delta_time > duration_since_last_call {
            std::thread::sleep(std::time::Duration::from_millis(
                (self.delta_time - duration_since_last_call) as u64,
            ));
        }
    }

    pub fn speed_info(&mut self, buffer: &mut Buffer, scene: &Scene) {
        let dur = self.times[((self.counter - 1) / 100) as usize];
        let mut fps = String::new();

        fps.push_str(&((1000.0 / dur).floor() as u32).to_string());
        fps.push_str("fps");

        let (x, y) = scene.scale(
            (2.0 * scene.scale) - (scene.fontsize * 0.2) as f64 * fps.len() as f64,
            2.0 * scene.scale - scene.fontsize as f64 * 0.5,
        );

        scene.draw_text(&fps, (x, y), buffer);
    }
}
