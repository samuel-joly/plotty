use std::collections::HashMap;

use ab_glyph::{Font, FontRef, Glyph};
use softbuffer::Buffer;

pub struct Scene {
    pub camera: (f64, f64, f64),
    pub screen: (f64, f64, f64),
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub font: HashMap<char, Vec<(u32, u32, u32)>>,
}

impl Scene {
    pub fn new(
        scale: f64,
        camera: (f64, f64, f64),
        screen: (f64, f64, f64),
        width: u32,
        height: u32,
    ) -> Scene {
        let font_ref = FontRef::try_from_slice(include_bytes!(
            "/home/azefortwo/.local/share/fonts/LibreBaskerville-Italic.otf"
        ))
        .unwrap();

        Scene {
            camera,
            screen,
            width,
            height,
            scale,
            font: Scene::compile_font(font_ref, 17.0),
        }
    }

    pub fn project(&self, points: Vec<(f64, f64, f64)>, normalize: bool) -> Vec<(f64, f64)> {
        let mut projection: Vec<(f64, f64)> = vec![];
        let dot_prod;
        if points.len() > 2 {
            let normal = self.normal_triangle(&points);
            dot_prod = (points[0].0 - self.camera.0) * normal.0
                + (points[0].1 - self.camera.1) * normal.1
                + (points[0].2 - self.camera.2) * normal.2;
        } else {
            dot_prod = 1.0;
        }
        if dot_prod < 0.0 || !normalize == true {
            for point in points {
                let mut x = ((point.0 - self.camera.0) * (self.screen.2 - self.camera.2)
                    / (point.2 - self.camera.2))
                    + self.camera.0;
                let mut y = ((point.1 - self.camera.1) * (self.screen.2 - self.camera.2)
                    / (point.2 - self.camera.2))
                    + self.camera.1;

                (x, y) = self.scale(x, y);
                projection.push((x, y));
            }
        }
        projection
    }

    pub fn scale(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (((x + self.scale) / (2.0 * self.scale)) * self.width as f64),
            ((1.0 - (y + self.scale) / (2.0 * self.scale)) * self.height as f64),
        )
    }

    pub fn draw_triangle(&self, triangle: Vec<(f64, f64)>, buffer: &mut Buffer) {
        let _colors = vec![0xFF0000, 0x00FF00, 0x0000FF];
        if triangle.len() < 2 {
            return;
        }

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

        for (_cnt, (dist_x, dist_y, triangle)) in dists.into_iter().enumerate() {
            let total_dist = dist_x.abs() + dist_y.abs();
            for j in 0..total_dist.floor() as i32 {
                let x = triangle.0 + dist_x * j as f64 / total_dist;
                let y = triangle.1 + dist_y * j as f64 / total_dist;
                let index = x.floor() as i32 + (y.floor() as i32 * self.width as i32);
                if buffer.get(index as usize).is_some() {
                    buffer[index as usize] = 0xFFFFFF;
                    //buffer[index as usize] = colors[_cnt as usize];
                }
            }
        }
    }

    pub fn normal_triangle(&self, triangle: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
        let x_ab: f64 = triangle[1].0 - triangle[0].0;
        let y_ab: f64 = triangle[1].1 - triangle[0].1;
        let z_ab: f64 = triangle[1].2 - triangle[0].2;

        let x_ac: f64 = triangle[2].0 - triangle[0].0;
        let y_ac: f64 = triangle[2].1 - triangle[0].1;
        let z_ac: f64 = triangle[2].2 - triangle[0].2;

        let normal_x = y_ab * z_ac - z_ab * y_ac;
        let normal_y = z_ab * x_ac - x_ab * z_ac;
        let normal_z = x_ab * y_ac - y_ab * x_ac;

        (normal_x, normal_y, normal_z)
    }

    pub fn draw_circle(
        &self,
        start_x: f64,
        start_y: f64,
        start_z: f64,
        radius: f64,
        color: u32,
        buffer: &mut Buffer,
    ) {
        for i in 0..628 {
            let x = ((i as f64 / 100.0).cos() * radius) + start_x;
            let y = ((i as f64 / 100.0).sin() * radius) + start_y;

            let (proj_x, proj_y) = self.project(vec![(x, y, start_z)], false)[0];
            let index = proj_x.floor() as i32 + (proj_y.floor() as i32 * self.width as i32);
            buffer[index as usize] = color;
        }
    }

    pub fn draw_text(&self, text: &str, x_start: f64, y_start: f64, buffer: &mut Buffer) {
        for letter in text.chars() {
            for pix in &self.font[&letter] {
                let x = pix.0 as f64 + x_start;
                let y = pix.1 as f64 + y_start;
                let (mut proj_x, mut proj_y) =
                    self.project(vec![(x, y, self.camera.2 + 1.0)], false)[0];
                (proj_x, proj_y) = self.scale(proj_x, proj_y);
                let index = proj_x.floor() as i32 + (proj_y.floor() as i32 * self.width as i32);
                dbg!(x, y, proj_x, proj_y, index);
                buffer[index as usize] = pix.2;
            }
        }
    }

    pub fn compile_font(font: FontRef, fontsize: f32) -> HashMap<char, Vec<(u32, u32, u32)>> {
        let str =
            String::from("1234567890abcdefghijklmnopqrstuvwxyz,-:.ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut symbols: HashMap<char, Vec<(u32, u32, u32)>> = HashMap::new();
        for s in str.chars() {
            let mut ret: Vec<(u32, u32, u32)> = vec![];
            let q_glyph: Glyph = font.glyph_id(s).with_scale(fontsize);

            if let Some(q) = font.outline_glyph(q_glyph) {
                q.draw(|x, mut y, c| {
                    if s == ',' || s == '.' {
                        y += 10;
                    }
                    if s == '-' {
                        y += 5;
                    }
                    let red = (255.0 * c).floor() as u32;
                    let color = red | (red << 8) | (red << 16);
                    ret.push((x, y, color));
                });
                symbols.insert(s, ret);
            }
        }
        symbols
    }
}
