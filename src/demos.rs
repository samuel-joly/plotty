use std::f64::consts::PI;

pub fn rotating_cube(pos: (f64, f64, f64), size: f64, fr: f64) -> Vec<Vec<Vec<(f64, f64, f64)>>> {
    let elevation = pos.1 + (2.0 * size * 0.7) ;
    vec![
        vec![vec![
            (pos.0 + fr.cos() * size, elevation, pos.2 + fr.sin() * size),
            (
              pos.0 + (fr + (PI / 2.0)).cos() * size,
                 elevation,
                pos.2 + (fr + (PI / 2.0)).sin() * size,
            ),
            (pos.0 - fr.cos() * size, elevation, pos.2 + -fr.sin() * size),
        ]],
        vec![vec![
            (pos.0 + fr.cos() * size, pos.1, pos.2 + fr.sin() * size),
            (
              pos.0 + (fr + (PI / 2.0)).cos() * size,
                pos.1,
                pos.2 + (fr + (PI / 2.0)).sin() * size,
            ),
            (pos.0 - fr.cos() * size, pos.1, pos.2 + -fr.sin() * size),
        ]],
        vec![
            vec![
                (pos.0 + fr.cos() * size, pos.1, pos.2 + fr.sin() * size),
                (pos.0 + fr.cos() * size, elevation, pos.2 + fr.sin() * size),
                (
                  pos.0 + (fr + (PI / 2.0)).cos() * size,
                     elevation,
                    pos.2 + (fr + (PI / 2.0)).sin() * size,
                ),
            ],
            vec![
                (pos.0 + fr.cos() * size, pos.1, pos.2 + fr.sin() * size),
                (
                  pos.0 + (fr + (PI / 2.0)).cos() * size,
                    pos.1,
                    pos.2 + (fr + (PI / 2.0)).sin() * size,
                ),
                (
                  pos.0 + (fr + (PI / 2.0)).cos() * size,
                     elevation,
                    pos.2 + (fr + (PI / 2.0)).sin() * size,
                ),
            ],
        ],
        vec![
            vec![
                (pos.0 - fr.cos() * size, pos.1, pos.2 + -fr.sin() * size),
                (pos.0 - fr.cos() * size, elevation, pos.2 + -fr.sin() * size),
                (
                  pos.0 - (fr + (PI / 2.0)).cos() * size,
                     elevation,
                    pos.2 + -(fr + (PI / 2.0)).sin() * size,
                ),
            ],
            vec![
                (pos.0 - fr.cos() * size, pos.1, pos.2 + -fr.sin() * size),
                (
                  pos.0 - (fr + (PI / 2.0)).cos() * size,
                    pos.1,
                    pos.2 + -(fr + (PI / 2.0)).sin() * size,
                ),
                (
                  pos.0 - (fr + (PI / 2.0)).cos() * size,
                     elevation,
                    pos.2 + -(fr + (PI / 2.0)).sin() * size,
                ),
            ],
        ],
        vec![
            vec![
                (pos.0 + fr.cos() * size, pos.1, pos.2 + fr.sin() * size),
                (pos.0 + fr.cos() * size, elevation, pos.2 + fr.sin() * size),
                (
                  pos.0 + (fr + (PI * 1.5)).cos() * size,
                     elevation,
                    pos.2 + (fr + (PI * 1.5)).sin() * size,
                ),
            ],
            vec![
                (pos.0 + fr.cos() * size, pos.1, pos.2 + fr.sin() * size),
                (
                  pos.0 + (fr + (PI * 1.5)).cos() * size,
                    pos.1,
                    pos.2 + (fr + (PI * 1.5)).sin() * size,
                ),
                (
                  pos.0 + (fr + (PI * 1.5)).cos() * size,
                     elevation,
                    pos.2 + (fr + (PI * 1.5)).sin() * size,
                ),
            ],
        ],
        vec![
            vec![
                (pos.0 - fr.cos() * size, pos.1, pos.2 + -fr.sin() * size),
                (pos.0 - fr.cos() * size, elevation, pos.2 + -fr.sin() * size),
                (
                  pos.0 - (fr + (PI * 1.5)).cos() * size,
                     elevation,
                    pos.2 + -(fr + (PI * 1.5)).sin() * size,
                ),
            ],
            vec![
                (pos.0 - fr.cos() * size, pos.1, pos.2 + -fr.sin() * size),
                (
                  pos.0 - (fr + (PI * 1.5)).cos() * size,
                    pos.1,
                    pos.2 + -(fr + (PI * 1.5)).sin() * size,
                ),
                (
                  pos.0 - (fr + (PI * 1.5)).cos() * size,
                     elevation,
                    pos.2 + -(fr + (PI * 1.5)).sin() * size,
                ),
            ],
        ],
    ]
}
