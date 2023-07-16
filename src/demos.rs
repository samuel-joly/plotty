use std::f64::consts::PI;

pub fn cube(fr: f64) -> Vec<Vec<Vec<(f64, f64, f64)>>> {
    let y = -0.25;
    let radius: f64 = 0.5;
    vec![
        vec![vec![
            (fr.cos() * radius, radius, 2.0 + fr.sin() * radius),
            (
                (fr + (PI / 2.0)).cos() * radius,
                radius,
                2.0 + (fr + (PI / 2.0)).sin() * radius,
            ),
            (-fr.cos() * radius, radius, 2.0 + -fr.sin() * radius),
        ]],
        vec![vec![
            (fr.cos() * radius, y, 2.0 + fr.sin() * radius),
            (
                (fr + (PI / 2.0)).cos() * radius,
                y,
                2.0 + (fr + (PI / 2.0)).sin() * radius,
            ),
            (-fr.cos() * radius, y, 2.0 + -fr.sin() * radius),
        ]],
        vec![
            vec![
                (fr.cos() * radius, y, 2.0 + fr.sin() * radius),
                (fr.cos() * radius, radius, 2.0 + fr.sin() * radius),
                (
                    (fr + (PI / 2.0)).cos() * radius,
                    radius,
                    (fr + (PI / 2.0)).sin() * radius + 2.0,
                ),
            ],
            vec![
                (fr.cos() * radius, y, 2.0 + fr.sin() * radius),
                (
                    (fr + (PI / 2.0)).cos() * radius,
                    y,
                    2.0 + (fr + (PI / 2.0)).sin() * radius,
                ),
                (
                    (fr + (PI / 2.0)).cos() * radius,
                    radius,
                    (fr + (PI / 2.0)).sin() * radius + 2.0,
                ),
            ],
        ],
        vec![
            vec![
                (-fr.cos() * radius, y, 2.0 + -fr.sin() * radius),
                (-fr.cos() * radius, radius, 2.0 + -fr.sin() * radius),
                (
                    -(fr + (PI / 2.0)).cos() * radius,
                    radius,
                    -(fr + (PI / 2.0)).sin() * radius + 2.0,
                ),
            ],
            vec![
                (-fr.cos() * radius, y, 2.0 + -fr.sin() * radius),
                (
                    -(fr + (PI / 2.0)).cos() * radius,
                    y,
                    2.0 + -(fr + (PI / 2.0)).sin() * radius,
                ),
                (
                    -(fr + (PI / 2.0)).cos() * radius,
                    radius,
                    -(fr + (PI / 2.0)).sin() * radius + 2.0,
                ),
            ],
        ],
        vec![
            vec![
                (fr.cos() * radius, y, 2.0 + fr.sin() * radius),
                (fr.cos() * radius, radius, 2.0 + fr.sin() * radius),
                (
                    (fr + (PI * 1.5)).cos() * radius,
                    radius,
                    (fr + (PI * 1.5)).sin() * radius + 2.0,
                ),
            ],
            vec![
                (fr.cos() * radius, y, 2.0 + fr.sin() * radius),
                (
                    (fr + (PI * 1.5)).cos() * radius,
                    y,
                    2.0 + (fr + (PI * 1.5)).sin() * radius,
                ),
                (
                    (fr + (PI * 1.5)).cos() * radius,
                    radius,
                    (fr + (PI * 1.5)).sin() * radius + 2.0,
                ),
            ],
        ],
        vec![
            vec![
                (-fr.cos() * radius, y, 2.0 + -fr.sin() * radius),
                (-fr.cos() * radius, radius, 2.0 + -fr.sin() * radius),
                (
                    -(fr + (PI * 1.5)).cos() * radius,
                    radius,
                    -(fr + (PI * 1.5)).sin() * radius + 2.0,
                ),
            ],
            vec![
                (-fr.cos() * radius, y, 2.0 + -fr.sin() * radius),
                (
                    -(fr + (PI * 1.5)).cos() * radius,
                    y,
                    2.0 + -(fr + (PI * 1.5)).sin() * radius,
                ),
                (
                    -(fr + (PI * 1.5)).cos() * radius,
                    radius,
                    -(fr + (PI * 1.5)).sin() * radius + 2.0,
                ),
            ],
        ],
    ]
}
