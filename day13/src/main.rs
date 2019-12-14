fn main() {
    let mut machine = intcode::Machine::from_file("13.input", None);
    let machine_output = machine.process(0);
    let screen: Vec<_> = machine_output.chunks(3).collect();
    let blocks: Vec<_> = screen.iter().filter(|v| v[2] == 2).collect();
    println!("{}", blocks.len());
}
