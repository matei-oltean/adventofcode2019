fn get(x: usize, y: usize, program: &intcode::Program) -> usize {
    let mut machine = intcode::Machine::new(program.clone(), None);
    machine.process(Some(x as isize));
    *machine.process(Some(y as isize)).first().unwrap() as usize
}

fn get_number(n_i: usize, n_j: usize, program: &intcode::Program) -> usize {
    (0..n_i)
        .map(|i| {
            (0..n_j)
                .map(|j| get(i, j, program))
                .filter(|n| *n == 1)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn put_box(b_x: usize, b_y: usize, program: &intcode::Program) -> usize {
    let mut y = b_y;
    let mut right = 0;
    while get(right, y, program) == 0 {
        right += 1;
    }
    loop {
        while get(right + 1, y, program) == 1 {
            right += 1;
        }
        if right >= b_x {
            let left = right - b_x + 1;
            if get(left, y, program) == 1 {
                if get(left, y + b_y - 1, program) == 1 {
                    return 10_000 * left + y;
                }
            }
        }
        y += 1;
    }
}

fn draw_map(s_x: usize, s_y: usize, n_x: usize, n_y: usize, program: &intcode::Program) {
    (s_y..n_y).for_each(|i| {
        let line = (s_x..n_x)
            .map(move |j| match get(j, i, program) {
                0 => ".",
                _ => "#",
            })
            .collect::<Vec<_>>()
            .join("");
        println!("{}", line);
    });
}

fn main() {
    let program = intcode::program_from_file("19.input");
    dbg!(get_number(50, 50, &program));
    dbg!(put_box(100, 100, &program));
}
