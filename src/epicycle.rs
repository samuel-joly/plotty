use crate::scene::Scene;
use softbuffer::Buffer;

pub fn draw_epicycle(
    main_pos: (f64, f64, f64),
    size: f64,
    buffer: &mut Buffer,
    scene: &Scene,
    frame: u32,
    pos: &mut Vec<(f64, f64)>,
) {
    let newframe = frame as f64 / 30.0;
    let epicycle_radius: f64 = size / 5.0;
    let planet_radius: f64 = size / 50.0;
    let planet_speed: f64 = 8.0;
    let epicycle_x = (newframe.cos() * size) + ((newframe * planet_speed).cos() * epicycle_radius);
    let epicycle_y = (newframe.sin() * size) + ((newframe * planet_speed).sin() * epicycle_radius);

    // Planet trajectory tail
    let trajectory_length = 50;
    let (proj_x, proj_y) = scene.project(
        vec![(epicycle_x + main_pos.0, epicycle_y + main_pos.1, main_pos.2)],
        false,
    )[0];
    if pos.len() < trajectory_length {
        pos.push((proj_x, proj_y));
    } else {
        pos[(frame as u32 % trajectory_length as u32) as usize] = (proj_x, proj_y);
    }
    for cnt in 0..pos.len() {
        if cnt < pos.len() - 1 && cnt > 0
            && cnt != (frame % trajectory_length as u32) as usize
            && cnt + 1 != (frame % trajectory_length as u32) as usize
            && cnt - 1 != (frame % trajectory_length as u32) as usize
        {
            scene.draw_line(
                (pos[cnt].0, pos[cnt].1),
                (pos[cnt + 1].0, pos[cnt + 1].1),
                0x00FF3C,
                buffer,
            );
        }
    }

    scene.draw_circle((main_pos.0, main_pos.1, main_pos.2), size, 0xFFFFFF, buffer);
    scene.draw_circle(
        (
            ((newframe).cos() * size) + main_pos.0,
            ((newframe).sin() * size) + main_pos.1,
            main_pos.2,
        ),
        epicycle_radius,
        0xFF0000,
        buffer,
    );
    scene.draw_circle(
        (epicycle_x + main_pos.0, epicycle_y + main_pos.1, main_pos.2),
        planet_radius,
        0x00FF00,
        buffer,
    );
}
