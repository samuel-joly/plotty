use crate::scene::Scene;
use softbuffer::Buffer;

pub fn draw_epicycle(
    main_pos: (f64, f64, f64),
    size: f64,
    buffer: &mut Buffer,
    scene: &Scene,
    frame: f64,
    pos:&mut Vec<u32>,
) {
    let epicycle: f64 = size / 5.0;
    let planet: f64 = size / 50.0;
    let planet_speed: f64 = 8.0;
    let epicycle_x = (frame.cos() * size) + ((frame * planet_speed).cos() * epicycle);
    let epicycle_y = (frame.sin() * size) + ((frame * planet_speed).sin() * epicycle);
    let (proj_x, proj_y) = scene.project(vec![(epicycle_x+main_pos.0, epicycle_y+main_pos.1, main_pos.2)], false)[0];
    let index_trajectory = proj_x.floor() as u32 + (proj_y.floor() as u32 * scene.width);
    let trailing_traj = 30;
    if pos.len() < trailing_traj {
        pos.push(index_trajectory);
        pos[0] += 1;
    } else {
        let mut i = pos[0] % trailing_traj as u32;
        if i == 0 { i = 1; }
        pos[i as usize] = index_trajectory;
        pos[0] += 1;
    }
    for traj in pos {
        if buffer.get(*traj as usize).is_some() {
            buffer[*traj as usize] = 0x00FF2C;
        }
    }


    scene.draw_circle((main_pos.0, main_pos.1, main_pos.2), size, 0xFFFFFF, buffer);
    scene.draw_circle(
        (
            ((frame).cos() * size) + main_pos.0 ,
            ((frame).sin() * size) + main_pos.1 ,
            main_pos.2,
        ),
        epicycle,
        0xFF0000,
        buffer,
    );
    scene.draw_circle(
        (epicycle_x + main_pos.0, epicycle_y + main_pos.1, main_pos.2),
        planet,
        0x00FF00,
        buffer,
    );
}
