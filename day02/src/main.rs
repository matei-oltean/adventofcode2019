use intcode::Machine;

fn main() {
    let mut machine = Machine::from_file("2.input", None);
    machine.set(1, 12);
    machine.set(2, 2);
    machine.process(Some(0));
    println!("{}", machine.get(0));
}
