use softbuffer::Buffer;

pub struct Scene {
    pub camera: (f64, f64, f64),
    pub screen: (f64, f64, f64),
    pub width: u32,
    pub height: u32,
}

impl Scene {
    pub fn scale(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (((x + 1.0) / 2.0) * self.width as f64),
            ((1.0 - (y + 1.0) / 2.0) * self.height as f64),
        )
    }

    pub fn project(&self, points: Vec<(f64, f64, f64)>) -> Vec<(f64, f64)> {
        let mut projection: Vec<(f64, f64)> = vec![];
        for point in points {
            let x = ((point.0 - self.camera.0) * (self.screen.2 - self.camera.2)
                / (point.2 - self.camera.2))
                + self.camera.0;
            let y = ((point.1 - self.camera.1) * (self.screen.2 - self.camera.2)
                / (point.2 - self.camera.2))
                + self.camera.1;
            projection.push((x, y));
        }
        projection
    }

    pub fn project_and_scale(&self, points: Vec<(f64, f64, f64)>) -> Vec<(f64, f64)> {
        let mut projection: Vec<(f64, f64)> = vec![];
        for point in points {
            let mut x = ((point.0 - self.camera.0) * (self.screen.2 - self.camera.2)
                / (point.2 - self.camera.2))
                + self.camera.0;
            let mut y = ((point.1 - self.camera.1) * (self.screen.2 - self.camera.2)
                / (point.2 - self.camera.2))
                + self.camera.1;

            x = ((x + 1.0) / 2.0) * self.width as f64;
            y = (1.0 - (y + 1.0) / 2.0) * self.height as f64;
            projection.push((x, y));
        }
        projection
    }

    pub fn draw_triangle(&self, triangle: Vec<(f64, f64)>, buffer: &mut Buffer) {
        let colors = vec![0xFF0000, 0x00FF00, 0x0000FF];

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
                let index = x.floor() as i32 + (y.floor() as i32 * self.width as i32);
                if buffer.get(index as usize).is_some() {
                    buffer[index as usize] = colors[cnt as usize];
                }
            }
        }
    }
}
