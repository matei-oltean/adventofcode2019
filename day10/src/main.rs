use std::collections::HashSet;

type Point = (i32, i32);

fn gcd(a: u32, b: u32) -> u32 {
    if a*b == 0 {
        a + b
    } else {
        let (a, b) = (a.max(b), a.min(b));
        gcd(b, a % b)
    }
}

fn find_angle((x, y): &Point, (xx, yy): &Point) -> Point {
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
        (dx/gcd, dy/gcd)
    }
}

fn find_number_of_asteroids(p: &Point, m: &[Point]) -> isize {
    let mut asteroids = HashSet::new();
    let mut result = 0;
    for point in m {
        let slope = find_angle(p, point);
        if asteroids.insert(slope) {
            result += 1
        }
    }
    result
}

fn main() {
    let map = reader::read_input("10.input");
    let asteroids: Vec<Point> = map
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, s)| {
                if s == '#' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    let max = asteroids
        .iter()
        .map(|p| find_number_of_asteroids(p, &asteroids))
        .max()
        .unwrap() - 1;
    println!("{:?}", max)
}
