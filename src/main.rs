use std::io::{self, Write};
use std::f32;

mod vec;

use crate::vec::V;

#[inline]
fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn random_val() -> f32 {
    rand::random()
}

fn box_test(position: V, lower_left: V, upper_right: V) -> f32 {
    let lower_left = position + lower_left * -1.0;
    let upper_right = upper_right + position * -1.0;
    -min(
        min(
            min(lower_left.x, upper_right.x),
            min(lower_left.y, upper_right.y),
        ),
        min(lower_left.z, upper_right.z),
    )
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Hit {
    None,
    Letter,
    Wall,
    Sun,
}

fn query_database(position: V) -> (Hit, f32) {
    let mut distance = f32::MAX;
    let mut f = position;
    f.z = 0.0;

    const LETTERS: &str = concat!(
        "5O5_", "5W9W", "5_9_", // P (without curve)
        "AOEO", "COC_", "A_E_", // I
        "IOQ_", "I_QO", // X
        "UOY_", "Y_]O", "WW[W", // A
        "aOa_", "aWeW", "a_e_", "cWiO" // R (without curve)
    );

    for chunk in LETTERS.as_bytes().chunks(4) {
        let begin = V::from((chunk[0] as f32 - 79.0, chunk[1] as f32 - 79.0)) * 0.5;
        let e = V::from((chunk[2] as f32 - 79.0, chunk[3] as f32 - 79.0)) * 0.5 + begin * -1.0;
        let o = f + (begin + e * min(-min((begin + f * -1.0) % e / (e % e), 0.0), 1.0)) * -1.0;
        distance = min(distance, o % o);
    }
    distance = distance.sqrt();

    let curves = [V::from((-11., 6.)), V::from((11., 6.))];
    for c in curves.iter().cloned() {
        let mut o = f + c * -1.0;
        distance = min(
            distance,
            if o.x > 0.0 {
                ((o % o).sqrt() - 2.0).abs()
            } else {
                o.y += if o.y > 0.0 {
                    f32::from(-2.0)
                } else {
                    f32::from(2.0)
                };
                (o % o).sqrt()
            },
        );
    }
    distance = (distance.powf(8.0) + position.z.powf(8.0)).powf(0.125) - 0.5;
    let mut hit = Hit::Letter;

    let roomdist = min(
        -min(
            box_test(
                position,
                V::from((-30., -0.5, -30.)),
                V::from((30., 18., 30.)),
            ),
            box_test(
                position,
                V::from((-25., 17., -25.)),
                V::from((25., 20., 25.)),
            ),
        ),
        box_test(
            V::from((position.x.abs() % 8., position.y, position.z)),
            V::from((1.5, 18.5, -25.)),
            V::from((6.5, 20., 25.)),
        ),
    );

    if roomdist < distance {
        distance = roomdist;
        hit = Hit::Wall;
    }

    let sun = f32::from(19.9) - position.y;
    if sun < distance {
        distance = sun;
        hit = Hit::Sun;
    }

    (hit, distance)
}

// Perform signed sphere marching
// Returns (hit, position, normal)
fn ray_marching(origin: V, direction: V) -> (Hit, V, V) {
    let mut no_hit_count = 0;

    let mut total_d = 0.0;

    while total_d < 100.0 {
        let pos = origin + direction * total_d;
        let (hit, d) = query_database(pos);
        let rayhit = if d < 0.01 {
            true
        } else {
            no_hit_count += 1;
            false
        };
        if rayhit || no_hit_count > 99 {
            let (_, nx) = query_database(pos + V::from((0.01, 0.0, 0.0)));
            let (_, ny) = query_database(pos + V::from((0.0, 0.01, 0.0)));
            let (_, nz) = query_database(pos + V::from((0.0, 0.0, 0.01)));
            return (hit, pos, !V::from((nx - d, ny - d, nz - d)));
        }

        total_d += d;
    }

    (Hit::None, V::default(), V::default())
}

fn trace(origin: V, direction: V) -> V {
    const BOUNCES: u32 = 3;
    let mut direction = direction;
    let mut origin = origin;
    let light_direction = !V::from((0.6, 0.6, 1.0));
    let mut attenuation = V::from(1.0);
    let mut color = V::default();

    for _ in 0..BOUNCES {
        let (hit, sampled_position, normal) = ray_marching(origin, direction);

        match hit {
            Hit::None => break,
            Hit::Letter => {
                direction = direction + normal * (normal % direction * -2.0);
                origin = sampled_position + direction * 0.1;
                attenuation = attenuation * 0.2;
            }
            Hit::Wall => {
                let incidence = normal % light_direction;
                let p = 6.283185 * random_val();
                let c = random_val();
                let s = (1.0 - c).sqrt();
                let g = normal.z.signum();
                let u = f32::from(-1.0) / (g + normal.z);
                let v = normal.x * normal.y * u;
                direction = V::from((v, g + normal.y * normal.y * u, -normal.y)) * (p.cos() * s)
                    + V::from((
                        f32::from(1.0) + g * normal.x * normal.x * u,
                        g * v,
                        -g * normal.x,
                    )) * (p.sin() * s)
                    + normal * c.sqrt();
                origin = sampled_position + direction * 0.1;
                attenuation = attenuation * 0.2;
                if incidence > 0.0 {
                    let (h, _p, _n) =
                        ray_marching(sampled_position + normal * 0.1, light_direction);
                    if h == Hit::Sun {
                        color = color + attenuation * V::from((500., 400., 100.)) * incidence;
                    }
                }
            }
            Hit::Sun => {
                color = color + attenuation * V::from((50., 80., 100.));
                break;
            }
        }
    }

    color
}

const W: i32 = 960;
const H: i32 = 540;
const SAMPLES_COUNT: u32 = 8;
const POSITION: V = V::new(-22., 5., 25.);

fn pixel(x: i32, y: i32, goal: V, left: V, up: V) -> [u8; 3] {
    let mut color = V::default();

    for _ in 0..SAMPLES_COUNT {
        color = color
            + trace(
                POSITION,
                !(goal
                    + left * ((x - W / 2) as f32 + random_val())
                    + up * ((y - H / 2) as f32 + random_val())),
            );
    }

    //eprintln!("x {} y {} color {:?}", x, y, color);

    color = color * (1. / (SAMPLES_COUNT as f32)) + 14. / 241.;
    let o = color + 1.0;
    let color = V::from((color.x / o.x, color.y / o.y, color.z * o.z)) * 255.0;

    [color.x as u8, color.y as u8, color.z as u8]
}

fn main() {
    let goal = !(V::new(-3., 4., 0.) + POSITION * -1.0);
    let left = !V::from((goal.z, 0.0, -goal.x)) * (1.0 / (W as f32));

    let up = V::from((
        goal.y * left.z - goal.z * left.y,
        goal.z * left.x - goal.x * left.z,
        goal.x * left.y - goal.y * left.x,
    ));

    print!("P6 {} {} 255 ", W, H);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let pixels = (0..H)
        .rev()
        .flat_map(|y| (0..W).rev().map(move |x| pixel(x, y, goal, left, up)));
    for pix in pixels {
        let _ = handle.write(&pix);
    }
}
