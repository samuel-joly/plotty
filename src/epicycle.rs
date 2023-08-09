use crate::scene::Scene;
use softbuffer::Buffer;

pub fn draw_epicycle(buffer: &mut Buffer, scene: &Scene, frame: f64) {
    let screen_z = 8.5;
    let main_orbit: f64 = 8.0;
    let epicycle: f64 = main_orbit / 5.0;
    let planet: f64 = main_orbit / 50.0;
    let planet_speed: f64 = 5.0;
    let epicycle_x = (frame.cos() * main_orbit) + ((frame * planet_speed).cos() * epicycle);
    let epicycle_y = (frame.sin() * main_orbit) + ((frame * planet_speed).sin() * epicycle);
    let (proj_x, proj_y) = scene.project(vec![(epicycle_x, epicycle_y, 3.0)], false)[0];
    let index_trajectory = proj_x.floor() as i32 + (proj_y.floor() as i32 * scene.width as i32);

    if buffer.get(index_trajectory as usize).is_some() {
        buffer[index_trajectory as usize] = 0x0000FF;
    }

    buffer.fill(0x000000);
    scene.draw_circle(10.0, 10.0, screen_z, main_orbit, 0xFFFFFF, buffer);
    scene.draw_circle(
        epicycle_x + 10.0,
        epicycle_y + 10.0,
        screen_z,
        planet,
        0x00FF00,
        buffer,
    );
    scene.draw_circle(
        ((frame).cos() * main_orbit) + 10.0,
        ((frame).sin() * main_orbit) + 10.0,
        screen_z,
        epicycle,
        0xFF0000,
        buffer,
    );
}
