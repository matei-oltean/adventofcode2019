use intcode::Machine;

fn main() {
    let mut machine = Machine::from_file("5.input", None);
    dbg!(machine.process(Some(5)));
}
