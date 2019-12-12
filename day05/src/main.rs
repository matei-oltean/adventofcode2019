use intcode::{CodeType, Machine, Program};

fn main() {
    let input: Program = reader::read_input("5.input")
        .split(",")
        .map(|c| c.parse::<CodeType>().unwrap())
        .enumerate()
        .map(|(i, c)| (i as CodeType, c))
        .collect();
    let mut machine = Machine::new(input, None);
    machine.process(5);
}
