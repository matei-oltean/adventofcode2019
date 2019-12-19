fn main() {
    let res = (0..50)
        .map(|i| {
            (0..50)
                .map(|j| {
                    let mut machine = intcode::Machine::from_file("19.input", None);
                    machine.process(Some(i));
                    *machine.process(Some(j)).first().unwrap()
                })
                .filter(|n| *n == 1)
                .sum::<isize>()
        })
        .sum::<isize>();
    println!("{}", res);
}
