use crate::scene::Scene;
use softbuffer::Buffer;

pub fn draw_epicycle(
    main_pos: (f64, f64, f64),
    size: f64,
    buffer: &mut Buffer,
    scene: &Scene,
    frame: f64,
) {
    let epicycle: f64 = size / 5.0;
    let planet: f64 = size / 50.0;
    let planet_speed: f64 = 5.0;
    let epicycle_x = (frame.cos() * size) + ((frame * planet_speed).cos() * epicycle);
    let epicycle_y = (frame.sin() * size) + ((frame * planet_speed).sin() * epicycle);
    let (proj_x, proj_y) = scene.project(vec![(epicycle_x, epicycle_y, main_pos.2)], false)[0];
    let index_trajectory = proj_x.floor() as i32 + (proj_y.floor() as i32 * scene.width as i32);

    if buffer.get(index_trajectory as usize).is_some() {
        buffer[index_trajectory as usize] = 0x0000FF;
    }

    scene.draw_circle((main_pos.0, main_pos.1, main_pos.2), size, 0xFFFFFF, buffer);
    scene.draw_circle(
        (epicycle_x + main_pos.0, epicycle_y + main_pos.1, main_pos.2),
        planet,
        0x00FF00,
        buffer,
    );
    scene.draw_circle(
        (
            ((frame).cos() * size) + main_pos.0,
            ((frame).sin() * size) + main_pos.1,
            main_pos.2,
        ),
        epicycle,
        0xFF0000,
        buffer,
    );
}
