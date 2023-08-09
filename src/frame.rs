use std::{
    collections::HashMap,
    io::Write,
    thread,
    time::{self, Instant},
};

pub struct Frame {
    pub counter: u32,
    pub timer: Instant,
    pub delta_time: u32,
    pub attributes: Vec<u32>,
    pub times: HashMap<u32, u32>,
}

impl Frame {
    pub fn new(delta_time: u32) -> Self {
        let mut fr = Self {
            counter: 0,
            timer: Instant::now(),
            delta_time,
            attributes: vec![],
            times: HashMap::new(),
        };
        // Last self time
        fr.attributes.push(0);
        // Mean of self time elapsed
        fr.attributes.push(1);
        fr
    }

    pub fn update(&mut self) {
        let duration_since_last_call = self.timer.elapsed().as_millis() as u32 - self.attributes[0];
        self.attributes[1] += duration_since_last_call;
        self.attributes[0] = self.timer.elapsed().as_millis() as u32;
        if self.delta_time > duration_since_last_call {
            std::thread::sleep(std::time::Duration::from_millis(
                (self.delta_time - duration_since_last_call) as u64,
            ));
        }
    }

    pub fn speed_info(&mut self) {
        let mut duration_since_last_update =
            self.timer.elapsed().as_millis() as u32 - self.attributes[0];

        duration_since_last_update += 1;
        print!(
            "\r{:?}: {:?}ms {:?}fps {:?}fps",
            self.counter,
            duration_since_last_update,
            (1000 / duration_since_last_update), // Theoritical fps
            1000 / (self.attributes[1] / self.counter), // Mean fps Weird result
        );
        let _ = std::io::stdout().flush();
    }
}
