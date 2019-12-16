use intcode::Machine;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32);
enum Direction {
    U,
    D,
    L,
    R,
}

fn next_point((x, y): &Point, d: &Direction) -> Point {
    match d {
        Direction::U => (*x, *y + 1),
        Direction::D => (*x, *y - 1),
        Direction::L => (*x - 1, *y),
        Direction::R => (*x + 1, *y),
    }
}

fn next_direction(d: &Direction, result: isize) -> Direction {
    if result == 0 {
        match d {
            Direction::U => Direction::L,
            Direction::L => Direction::D,
            Direction::D => Direction::R,
            Direction::R => Direction::U,
        }
    } else {
        match d {
            Direction::U => Direction::R,
            Direction::R => Direction::D,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
        }
    }
}

fn paint(machine: &mut Machine, initial_colour: isize) -> HashMap<Point, isize> {
    let mut direction = Direction::U;
    let mut pt = (0 as i32, 0 as i32);
    let mut result = HashMap::new();
    result.insert(pt, initial_colour);
    loop {
        let colour = match result.get(&pt) {
            None => 0,
            Some(x) => *x,
        };
        let op_result = machine.process(Some(colour));
        if op_result.len() == 0 {
            return result;
        }
        let (c, d) = (op_result[0], op_result[1]);
        result.insert(pt, c);
        direction = next_direction(&direction, d);
        pt = next_point(&pt, &direction);
    }
}

fn main() {
    let mut machine = Machine::from_file("11.input", None);
    let painted = paint(&mut machine, 1);
    let filtered: HashSet<_> = painted
        .iter()
        .filter(|(_, c)| if **c == 0 { false } else { true })
        .map(|(p, _)| p)
        .collect();
    let (min_x, min_y): Point = filtered
        .iter()
        .fold((i32::max_value(), i32::max_value()), |(mx, my), (x, y)| {
            (mx.min(*x), my.min(*y))
        });
    let normalised: HashSet<_> = filtered
        .iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect();

    let (max_x, max_y): Point = normalised
        .iter()
        .fold((i32::min_value(), i32::min_value()), |(mx, my), (x, y)| {
            (mx.max(*x), my.max(*y))
        });

    let mut s = String::new();
    for i in (0..max_y + 1).rev() {
        for j in 0..max_x + 1 {
            if normalised.contains(&(j, i)) {
                s.push('█');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}
