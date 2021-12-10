use std::collections::HashSet;

type Point = (i32, i32);
type Direction = isize;

fn next_point((x, y): &Point, d: Direction) -> Point {
    match d {
        1 => (*x, *y + 1),
        2 => (*x, *y - 1),
        3 => (*x - 1, *y),
        _ => (*x + 1, *y),
    }
}

fn prev_point(p: &Point, d: Direction) -> (Point, Direction) {
    let prev_dir = match d {
        1 => 2,
        2 => 1,
        3 => 4,
        _ => 3,
    };
    (next_point(p, prev_dir), prev_dir)
}

fn do_dfs(machine: &mut intcode::Machine) -> usize {
    let mut curr_pt = (0, 0);
    let mut visited = HashSet::new();
    let mut route = Vec::new();
    let directions = [1, 2, 3, 4];
    'outer: loop {
        visited.insert(curr_pt);
        for &dir in directions.iter() {
            let next = next_point(&curr_pt, dir);
            if visited.contains(&next) {
                continue;
            }
            let res = machine.process(Some(dir));
            let mv = res.first().unwrap();
            if mv == &0 {
                continue;
            }
            if mv == &2 {
                return route.len() + 1;
            }
            curr_pt = next;
            route.push(dir);
            continue 'outer;
        }
        loop {
            let prev = route.pop().unwrap();
            let (p, d) = prev_point(&curr_pt, prev);
            curr_pt = p;
            machine.process(Some(d));
            if prev != 4 {
                break;
            }
        }
    }
}

fn get_furthest(machine: &mut intcode::Machine) -> usize {
    let mut curr_pt = (0, 0);
    let mut res = 0;
    let mut visited = HashSet::new();
    let mut route = Vec::new();
    let directions = [1, 2, 3, 4];
    'outer: loop {
        visited.insert(curr_pt);
        for &dir in directions.iter() {
            let next = next_point(&curr_pt, dir);
            if visited.contains(&next) {
                continue;
            }
            let res = machine.process(Some(dir));
            let mv = res.first().unwrap();
            if mv == &0 {
                continue;
            }
            curr_pt = next;
            route.push(dir);
            continue 'outer;
        }
        res = res.max(route.len());
        loop {
            if route.is_empty() {
                return res;
            }
            let prev = route.pop().unwrap();
            let (p, d) = prev_point(&curr_pt, prev);
            curr_pt = p;
            machine.process(Some(d));
            if prev != 4 {
                break;
            }
        }
    }
}

fn main() {
    let mut machine = intcode::Machine::from_file("15.input", None);
    println!("{}", do_dfs(&mut machine));
    println!("{}", get_furthest(&mut machine));
}
