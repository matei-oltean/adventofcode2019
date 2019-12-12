use intcode::Machine;

fn main() {
    let mut machine = Machine::from_file("9.input", None);
    machine.process(1);
    let mut machine = Machine::from_file("9.input", None);
    machine.process(2);
}