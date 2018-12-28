#![warn(clippy::all)]

use std::f32;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use rayon::prelude::*;

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
            min(lower_left.x(), upper_right.x()),
            min(lower_left.y(), upper_right.y()),
        ),
        min(lower_left.z(), upper_right.z()),
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
    let f = V::new(position.x(), position.y(), 0.0);

    const LETTERS: &str = concat!(
        "5O5_", "5W9W", "5_9_", // P (without curve)
        "AOEO", "COC_", "A_E_", // I
        "IOQ_", "I_QO", // X
        "UOY_", "Y_]O", "WW[W", // A
        "aOa_", "aWeW", "a_e_", "cWiO" // R (without curve)
    );

    fn l(v: u8) -> f32 {
        f32::from(v) - 79.0
    }

    let mut distance = LETTERS
        .as_bytes()
        .chunks_exact(4)
        .map(|chunk| {
            (
                V::from((l(chunk[0]), l(chunk[1]))) * 0.5,
                V::from((l(chunk[2]), l(chunk[3]))) * 0.5,
            )
        })
        .map(|(begin, end)| (begin, end + begin * -1.0))
        .map(|(begin, e)| {
            let o = f + (begin + e * min(-min((begin + f * -1.0) % e / (e % e), 0.0), 1.0)) * -1.0;
            o % o
        })
        .fold(f32::MAX, min)
        .sqrt();

    let curves = &[V::from((-11., 6.)), V::from((11., 6.))];
    for c in curves {
        let mut o = f + *c * -1.0;
        distance = min(
            distance,
            if o.x() > 0.0 {
                ((o % o).sqrt() - 2.0).abs()
            } else {
                *o.y_mut() += if o.y() > 0.0 { -2.0 } else { 2.0 };
                (o % o).sqrt()
            },
        );
    }
    distance = (distance.powf(8.0) + position.z().powf(8.0)).powf(0.125) - 0.5;
    let mut hit = Hit::Letter;

    let roomdist = min(
        -min(
            box_test(position, V::new(-30., -0.5, -30.), V::new(30., 18., 30.)),
            box_test(position, V::new(-25., 17., -25.), V::new(25., 20., 25.)),
        ),
        box_test(
            V::new(position.x().abs() % 8., position.y(), position.z()),
            V::new(1.5, 18.5, -25.),
            V::new(6.5, 20., 25.),
        ),
    );

    if roomdist < distance {
        distance = roomdist;
        hit = Hit::Wall;
    }

    let sun = 19.9 - position.y();
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
            let (_, nx) = query_database(pos + V::new(0.01, 0.0, 0.0));
            let (_, ny) = query_database(pos + V::new(0.0, 0.01, 0.0));
            let (_, nz) = query_database(pos + V::new(0.0, 0.0, 0.01));
            return (hit, pos, !V::new(nx - d, ny - d, nz - d));
        }

        total_d += d;
    }

    (Hit::None, V::default(), V::default())
}

fn trace(origin: V, direction: V) -> V {
    const BOUNCES: u32 = 3;
    let mut direction = direction;
    let mut origin = origin;
    let light_direction = !V::new(0.6, 0.6, 1.0);
    let mut attenuation = V::from(1.0);
    let mut color = V::default();

    for _ in 0..BOUNCES {
        let (hit, sampled_position, normal) = ray_marching(origin, direction);

        match hit {
            Hit::None => break,
            Hit::Letter => {
                direction = direction + normal * (normal % direction * -2.0);
                origin = sampled_position + direction * 0.1;
                attenuation *= 0.2;
            }
            Hit::Wall => {
                let incidence = normal % light_direction;
                let p = 6.283_185 * random_val();
                let c = random_val();
                let s = (1.0 - c).sqrt();
                let g = normal.z().signum();
                let u = -1.0 / (g + normal.z());
                let v = normal.x() * normal.y() * u;
                direction = V::new(v, g + normal.y() * normal.y() * u, -normal.y()) * (p.cos() * s)
                    + V::new(
                        1.0 + g * normal.x() * normal.x() * u,
                        g * v,
                        -g * normal.x(),
                    ) * (p.sin() * s)
                    + normal * c.sqrt();
                origin = sampled_position + direction * 0.1;
                attenuation *= 0.2;
                if incidence > 0.0 {
                    let (h, _p, _n) =
                        ray_marching(sampled_position + normal * 0.1, light_direction);
                    if h == Hit::Sun {
                        color += attenuation * V::new(500., 400., 100.) * incidence;
                    }
                }
            }
            Hit::Sun => {
                color += attenuation * V::new(50., 80., 100.);
                break;
            }
        }
    }

    color
}

const W: i32 = 960;
const H: i32 = 540;
const POSITION: V = V::new(-22., 5., 25.);

fn sample(x: i32, y: i32, pos: V, goal: V, left: V, up: V) -> V {
    trace(
        pos,
        !(goal
            + left * ((x - W / 2) as f32 + random_val())
            + up * ((y - H / 2) as f32 + random_val())),
    )
}

// Reinhard tone mapping
fn tone_map(samples: u32, color: V) -> [u8; 3] {
    let mut color = color;

    color = color * (1. / (samples as f32)) + 14. / 241.;
    let o = color + 1.0;
    let color = V::new(color.x() / o.x(), color.y() / o.y(), color.z() / o.z()) * 255.0;

    [color.x() as u8, color.y() as u8, color.z() as u8]
}

fn coords() -> impl ParallelIterator<Item = (i32, i32)> {
    (0..H)
        .into_par_iter()
        .flat_map(|y| (0..W).into_par_iter().map(move |x| (W - x - 1, H - y - 1)))
}

fn main() {
    // These are really constants, but Rust constfn can't deal with them yet.
    let goal = !(V::new(-3., 4., 0.) + POSITION * -1.0);
    let left = !V::new(goal.z(), 0.0, -goal.x()) * (1.0 / (W as f32));
    // Cross-product to get the up vector
    let up = goal.cross(left);

    let mut frame: Vec<_> = coords().map(|_| V::default()).collect();

    for s in 1..=32 {
        let start = Instant::now();

        let pixels: Vec<_> = coords()
            .map(move |(x, y)| sample(x, y, POSITION, goal, left, up))
            .collect();

        frame.par_iter_mut().zip(pixels).for_each(|(f, p)| *f += p);

        let file = File::create(format!("out-{}.ppm", s)).expect("create failed");
        let mut handle = BufWriter::new(file);

        println!("Sample {} took {:?}", s, start.elapsed());

        let _ = write!(handle, "P6 {} {} 255 ", W, H);

        for pix in frame.iter().map(|pix| tone_map(s, *pix)) {
            let _ = handle.write(&pix);
        }
    }
} // Andrew Kensler
