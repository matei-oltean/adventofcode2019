fn get_blocks(screen: &[&[isize]]) -> usize {
    screen
        .iter()
        .filter(|v| v[2] == 2)
        .collect::<Vec<_>>()
        .len()
}

fn get_item_coords(screen: &[&[isize]], id: isize) -> (isize, isize) {
    match screen
        .iter()
        .filter(|v| v[2] == id)
        .collect::<Vec<_>>()
        .first()
    {
        None => (0 as isize, 0 as isize),
        Some(t) => (t[0], t[1]),
    }
}

fn try_get_score(screen: &[&[isize]]) -> Option<isize> {
    match screen
        .iter()
        .filter(|t| (t[0] == -1) && (t[1] == 0))
        .collect::<Vec<_>>()
        .first()
    {
        None => None,
        Some(t) => Some(t[2]),
    }
}

fn get_score(machine: &mut intcode::Machine) {
    let mut next_instruction = None;
    loop {
        let machine_output = machine.process(next_instruction);
        let screen: Vec<_> = machine_output.chunks(3).collect();
        match try_get_score(&screen) {
            Some(n) => {
                dbg!(n);
            }
            None => (),
        }
        next_instruction = match get_item_coords(&screen, 3).0 - get_item_coords(&screen, 4).0 {
            0 => Some(0),
            n => {
                if n < 0 {
                    Some(1)
                } else {
                    Some(-1)
                }
            }
        };
    }
}

fn main() {
    let mut machine = intcode::Machine::from_file("13.input", None);
    machine.set(0, 2);
    get_score(&mut machine);
}
