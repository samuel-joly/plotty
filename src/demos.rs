use std::f64::consts::PI;

pub fn rotating_cube(pos: (f64, f64, f64), size: f64, fr: f64) -> Vec<Vec<Vec<(f64, f64, f64)>>> {
    let elevation = pos.1 +  size*2.0;
    vec![
        vec![
            vec![
                (
                    pos.0 + (fr + (PI / 2.0)).cos() * (size*1.3),
                    elevation,
                    pos.2 + (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
                (pos.0 + fr.cos() * (size*1.3), elevation, pos.2 + fr.sin() * (size*1.3)),
                (pos.0 - fr.cos() * (size*1.3), elevation, pos.2 - fr.sin() * (size*1.3)),
            ],
            vec![
                (
                    pos.0 - (fr + (PI / 2.0)).cos() * (size*1.3),
                    elevation,
                    pos.2 - (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
                (pos.0 - fr.cos() * (size*1.3), elevation, pos.2 - fr.sin() * (size*1.3)),
                (pos.0 + fr.cos() * (size*1.3), elevation, pos.2 + fr.sin() * (size*1.3)),
            ],
        ],
        vec![
            vec![
                (pos.0 - fr.cos() * (size*1.3), pos.1, pos.2 - fr.sin() * (size*1.3)),
                (
                    pos.0 - (fr + (PI / 2.0)).cos() * (size*1.3),
                    pos.1,
                    pos.2 - (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
                (pos.0 + fr.cos() * (size*1.3), pos.1, pos.2 + fr.sin() * (size*1.3)),
            ],
            vec![
                (pos.0 + fr.cos() * (size*1.3), pos.1, pos.2 + fr.sin() * (size*1.3)),
                (
                    pos.0 + (fr + (PI / 2.0)).cos() * (size*1.3),
                    pos.1,
                    pos.2 + (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
                (pos.0 - fr.cos() * (size*1.3), pos.1, pos.2 + -fr.sin() * (size*1.3)),
            ],
        ],
        vec![
            vec![
                (pos.0 + fr.cos() * (size*1.3), pos.1, pos.2 + fr.sin() * (size*1.3)),
                (pos.0 + fr.cos() * (size*1.3), elevation, pos.2 + fr.sin() * (size*1.3)),
                (
                    pos.0 + (fr + (PI / 2.0)).cos() * (size*1.3),
                    elevation,
                    pos.2 + (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
            ],
            vec![
                (
                    pos.0 + (fr + (PI / 2.0)).cos() * (size*1.3),
                    pos.1,
                    pos.2 + (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
                (pos.0 + fr.cos() * (size*1.3), pos.1, pos.2 + fr.sin() * (size*1.3)),
                (
                    pos.0 + (fr + (PI / 2.0)).cos() * (size*1.3),
                    elevation,
                    pos.2 + (fr + (PI / 2.0)).sin() * (size*1.3),
                ),
            ],
        ],
        vec![
            vec![
                (pos.0 - fr.cos() * (size*1.3), pos.1, pos.2 + -fr.sin() * (size*1.3)),
                (pos.0 - fr.cos() * (size*1.3), elevation, pos.2 + -fr.sin() * (size*1.3)),
                (
                    pos.0 - (fr + (PI / 2.0)).cos() * (size*1.3),
                    elevation,
                    pos.2 + -(fr + (PI / 2.0)).sin() * (size*1.3),
                ),
            ],
            vec![
                (
                    pos.0 - (fr + (PI / 2.0)).cos() * (size*1.3),
                    pos.1,
                    pos.2 + -(fr + (PI / 2.0)).sin() * (size*1.3),
                ),
                (pos.0 - fr.cos() * (size*1.3), pos.1, pos.2 + -fr.sin() * (size*1.3)),
                (
                    pos.0 - (fr + (PI / 2.0)).cos() * (size*1.3),
                    elevation,
                    pos.2 + -(fr + (PI / 2.0)).sin() * (size*1.3),
                ),
            ],
        ],
        vec![
            vec![
                (pos.0 + fr.cos() * (size*1.3), elevation, pos.2 + fr.sin() * (size*1.3)),
                (pos.0 + fr.cos() * (size*1.3), pos.1, pos.2 + fr.sin() * (size*1.3)),
                (
                    pos.0 + (fr + (PI * 1.5)).cos() * (size*1.3),
                    elevation,
                    pos.2 + (fr + (PI * 1.5)).sin() * (size*1.3),
                ),
            ],
            vec![
                (
                    pos.0 + (fr + (PI * 1.5)).cos() * (size*1.3),
                    pos.1,
                    pos.2 + (fr + (PI * 1.5)).sin() * (size*1.3),
                ),
                (
                    pos.0 + (fr + (PI * 1.5)).cos() * (size*1.3),
                    elevation,
                    pos.2 + (fr + (PI * 1.5)).sin() * (size*1.3),
                ),
                (pos.0 + fr.cos() * (size*1.3), pos.1, pos.2 + fr.sin() * (size*1.3)),
            ],
        ],
        vec![
            vec![
                (pos.0 - fr.cos() * (size*1.3), elevation, pos.2 + -fr.sin() * (size*1.3)),
                (pos.0 - fr.cos() * (size*1.3), pos.1, pos.2 + -fr.sin() * (size*1.3)),
                (
                    pos.0 - (fr + (PI * 1.5)).cos() * (size*1.3),
                    elevation,
                    pos.2 + -(fr + (PI * 1.5)).sin() * (size*1.3),
                ),
            ],
            vec![
                (
                    pos.0 - (fr + (PI * 1.5)).cos() * (size*1.3),
                    pos.1,
                    pos.2 + -(fr + (PI * 1.5)).sin() * (size*1.3),
                ),
                (
                    pos.0 - (fr + (PI * 1.5)).cos() * (size*1.3),
                    elevation,
                    pos.2 + -(fr + (PI * 1.5)).sin() * (size*1.3),
                ),
                (pos.0 - fr.cos() * (size*1.3), pos.1, pos.2 + -fr.sin() * (size*1.3)),
            ],
        ],
    ]
}

pub fn cube(pos: (f64, f64, f64), size: f64) -> Vec<Vec<Vec<(f64, f64, f64)>>> {
    vec![
        vec![
            vec![
                (pos.0 + size, pos.1 - size, pos.2),
                (pos.0 + size, pos.1 + size, pos.2),
                (pos.0 + size, pos.1 + size, pos.2 + size),
            ],
            vec![
                (pos.0 + size, pos.1 + size, pos.2 + size),
                (pos.0 + size, pos.1 - size, pos.2 + size),
                (pos.0 + size, pos.1 - size, pos.2),
            ],
        ],
        vec![
            vec![
                (pos.0 - size, pos.1 + size, pos.2),
                (pos.0 - size, pos.1 - size, pos.2),
                (pos.0 + size, pos.1 + size, pos.2),
            ],
            vec![
                (pos.0 - size, pos.1 - size, pos.2),
                (pos.0 + size, pos.1 - size, pos.2),
                (pos.0 + size, pos.1 + size, pos.2),
            ],
        ],
        vec![
            vec![
                (pos.0 - size, pos.1 - size, pos.2 + size),
                (pos.0 - size, pos.1 + size, pos.2 + size),
                (pos.0 + size, pos.1 + size, pos.2 + size),
            ],
            vec![
                (pos.0 + size, pos.1 + size, pos.2 + size),
                (pos.0 + size, pos.1 - size, pos.2 + size),
                (pos.0 - size, pos.1 - size, pos.2 + size),
            ],
        ],
        vec![
            vec![
                (pos.0 - size, pos.1 + size, pos.2),
                (pos.0 - size, pos.1 - size, pos.2),
                (pos.0 - size, pos.1 + size, pos.2 + size),
            ],
            vec![
                (pos.0 - size, pos.1 - size, pos.2),
                (pos.0 - size, pos.1 - size, pos.2 + size),
                (pos.0 - size, pos.1 + size, pos.2 + size),
            ],
        ],
        vec![
            vec![
                (pos.0 + size, pos.1 + size, pos.2),
                (pos.0 + size, pos.1 + size, pos.2 + size),
                (pos.0 - size, pos.1 + size, pos.2),
            ],
            vec![
                (pos.0 - size, pos.1 + size, pos.2),
                (pos.0 - size, pos.1 + size, pos.2 + size),
                (pos.0 + size, pos.1 + size, pos.2 + size),
            ],
        ],
        vec![
            vec![
                (pos.0 - size, pos.1 - size, pos.2),
                (pos.0 + size, pos.1 - size, pos.2),
                (pos.0 - size, pos.1 - size, pos.2 + size),
            ],
            vec![
                (pos.0 + size, pos.1 - size, pos.2 + size),
                (pos.0 - size, pos.1 - size, pos.2),
                (pos.0 + size, pos.1 - size, pos.2),
            ],
        ],
    ]
}
