use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

type Point = (i32, i32);
type Angle = f32;
type Slope = (i32, i32);

fn gcd(a: u32, b: u32) -> u32 {
    if a * b == 0 {
        a + b
    } else {
        let (a, b) = (a.max(b), a.min(b));
        gcd(b, a % b)
    }
}

fn dist((x, y): &Point, (xx, yy): &Point) -> u32 {
    ((xx - x).abs() + (yy - y).abs()) as u32
}

fn slope_to_angle((x, y): &Slope) -> Angle {
    if *x == 0 {
        (-*y as f32) * PI
    } else if *x > 0 {
        (-*y as f32 / *x as f32).atan()
    } else {
        (-*y as f32 / *x as f32).atan() - PI
    }
}

fn find_slope((x, y): &Point, (xx, yy): &Point) -> Slope {
    fn sign(d: i32) -> i32 {
        if d == 0 {
            0
        } else {
            d.abs() / d
        }
    }
    if x == xx {
        (0, sign(yy - y))
    } else if y == yy {
        (sign(xx - x), 0)
    } else {
        let (dx, dy) = (xx - x, yy - y);
        let gcd = gcd(dx.abs() as u32, dy.abs() as u32) as i32;
        (dx / gcd, dy / gcd)
    }
}

fn find_number_of_asteroids(p: &Point, m: &[Point]) -> isize {
    let mut asteroids = HashSet::new();
    let mut result = 0;
    for point in m {
        let slope = find_slope(p, point);
        if asteroids.insert(slope) {
            result += 1
        }
    }
    result
}

fn find_asteroid_circle(p: &Point, m: &HashSet<&Point>) -> Vec<(Point, Angle)> {
    let mut asteroids_at_angle = HashMap::new();
    for point in m {
        let slope = find_slope(p, point);
        let dist = dist(p, point);
        if dist == 0 {
            continue;
        }
        match asteroids_at_angle.get(&slope) {
            None => {
                asteroids_at_angle.insert(slope, (point, dist));
            }
            Some((_, d)) => {
                if dist < *d {
                    asteroids_at_angle.insert(slope, (point, dist));
                }
            }
        }
    }
    asteroids_at_angle
        .iter()
        .map(|(slope, (pt, _))| (***pt, slope_to_angle(slope)))
        .collect()
}

fn nth_asteroid(start_point: &Point, n: &mut usize, m: &[Point]) -> Point {
    let mut asteroid_map: HashSet<_> = m.iter().collect();
    loop {
        let asteroid_ring = find_asteroid_circle(start_point, &asteroid_map);
        if asteroid_ring.len() < *n {
            *n -= asteroid_ring.len();
            asteroid_ring.iter().for_each(|(p, _)| {
                asteroid_map.remove(p);
            });
        } else {
            let mut destroyed = asteroid_ring
                .iter()
                .map(|(p, angle)| (p, -angle + PI / 2.))
                .collect::<Vec<_>>();
            destroyed.sort_by(|(_, angle1), (_, angle2)| angle1.partial_cmp(angle2).unwrap());
            let (pt, _) = destroyed[*n - 1];
            return *pt;
        }
    }
}

fn main() {
    let map = reader::read_input("10.input");
    let asteroids: Vec<Point> = map
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, s)| {
                if s == '#' {
                    Some((j as i32, i as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    let (max_nb, max_pt) = asteroids
        .iter()
        .map(|p| (find_number_of_asteroids(p, &asteroids), p))
        .max_by_key(|(n, _)| *n)
        .unwrap();
    println!("{}", max_nb - 1);
    let (x, y) = nth_asteroid(max_pt, &mut 200, &asteroids);
    println!("{}", 100 * x + y);
}
