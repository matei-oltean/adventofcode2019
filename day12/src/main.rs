use std::collections::HashMap;
use std::str::FromStr;
type Position = (isize, isize, isize);

fn get_ith(i: i32, (x, y, z): &Position) -> isize {
    if i == 0 {
        return *x;
    }
    if i == 1 {
        return *y;
    }
    *z
}

fn times((x, y, z): &mut Position, n: isize) {
    *x *= n;
    *y *= n;
    *z *= n;
}

fn update_position((x, y, z): &mut Position, (dx, dy, dz): &Position) {
    if dx > &0 {
        *x += 1;
    } else if dx < &0 {
        *x -= 1;
    }
    if dy > &0 {
        *y += 1;
    } else if dy < &0 {
        *y -= 1;
    }
    if dz > &0 {
        *z += 1;
    } else if dz < &0 {
        *z -= 1;
    }
}

fn energy((x, y, z): &Position) -> isize {
    x.abs() + y.abs() + z.abs()
}

fn diff((x, y, z): &mut Position, (dx, dy, dz): &Position) {
    *x += dx;
    *y += dy;
    *z += dz;
}

fn get_energy(steps: isize, planets: &mut [Position]) -> isize {
    let mut velocities: Vec<Position> = (0..planets.len())
        .map(|_| (0 as isize, 0 as isize, 0 as isize))
        .collect();
    for _ in 0..steps {
        for (i, (x, y, z)) in planets.iter().enumerate() {
            for j in i + 1..planets.len() {
                let (xx, yy, zz) = planets[j];
                let mut diff = (xx - x, yy - y, zz - z);
                update_position(&mut velocities[i], &diff);
                times(&mut diff, -1 as isize);
                update_position(&mut velocities[j], &diff);
            }
        }
        for (i, mut p) in planets.iter_mut().enumerate() {
            diff(&mut p, &velocities[i]);
        }
    }
    (0..planets.len())
        .map(|i| energy(&planets[i]) * energy(&velocities[i]))
        .sum()
}

fn get_cycle(planets: &[Position]) -> usize {
    fn ppcm(a: usize, b: usize) -> usize {
        fn gcd(a: usize, b: usize) -> usize {
            if a * b == 0 {
                a + b
            } else {
                let (a, b) = (a.max(b), a.min(b));
                gcd(b, a % b)
            }
        };
        a * b / gcd(a, b)
    };
    let mut res = 0;
    let mut new_positions: Vec<_> = planets.iter().cloned().collect();
    let mut velocities: Vec<Position> = (0..planets.len())
        .map(|_| (0 as isize, 0 as isize, 0 as isize))
        .collect();
    let mut found: usize = 0;
    let mut cycles = HashMap::new();

    loop {
        for (i, (x, y, z)) in new_positions.iter().enumerate() {
            for j in i + 1..new_positions.len() {
                let (xx, yy, zz) = new_positions[j];
                let mut diff = (xx - x, yy - y, zz - z);
                update_position(&mut velocities[i], &diff);
                times(&mut diff, -1 as isize);
                update_position(&mut velocities[j], &diff);
            }
        }
        for (i, mut p) in new_positions.iter_mut().enumerate() {
            diff(&mut p, &velocities[i]);
        }

        res += 1;

        for i in 0..3 {
            if (0..3).all(|j| {
                (get_ith(i, &velocities[j]) == 0)
                    && (get_ith(i, &planets[j])) == get_ith(i, &new_positions[j])
            }) {
                if !(cycles.contains_key(&i)) {
                    cycles.insert(i, res);
                    found += 1;
                }
            }
        }
        if found == 3 {
            return cycles.values().fold(1 as usize, |p, x| ppcm(p, *x));
        }
    }
}

fn main() {
    let input = reader::read_input("12.input");
    let system: Vec<_> = input
        .lines()
        .map(|s| {
            let pos = String::from_str(s)
                .unwrap()
                .split(',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (pos[0], pos[1], pos[2])
        })
        .collect();
    //let energy = get_energy(1_000, &mut system);
    //println!("{}", energy);
    let cycle = get_cycle(&system);
    println!("{}", cycle);
}
