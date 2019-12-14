use std::str::FromStr;
type Position = (isize, isize, isize);

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

fn main() {
    let input = reader::read_input("12.input");
    let mut system: Vec<_> = input
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
    let energy = get_energy(1_000, &mut system);
    println!("{}", energy);
}
