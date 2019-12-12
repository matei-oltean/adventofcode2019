use intcode::{CodeType, Machine, Program};

fn main() {
    let input: Program = reader::read_input("2.input")
        .split(",")
        .map(|c| c.parse::<CodeType>().unwrap())
        .enumerate()
        .map(|(i, c)| (i as CodeType, c))
        .collect();
    let mut machine = Machine::new(input, None);
    machine.set(1, 12);
    machine.set(2, 2);
    machine.process(0);
    println!("{}", machine.get(0));
}
