use intcode::Machine;

fn main() {
    let mut machine = Machine::from_file("9.input", None);
    machine.process(Some(1));
    let mut machine = Machine::from_file("9.input", None);
    machine.process(Some(2));
}