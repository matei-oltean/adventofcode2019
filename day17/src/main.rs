fn main() {
    let mut machine = intcode::Machine::from_file("17.input", None);
    let mut scaffolds = machine.process(None);
    for &mut i in scaffolds.iter_mut() {
        if i == 10 {
            println!("");
        } else if i == 46 {
            print!(".");
        } else {
            print!("#");
        }
    }
    let map: Vec<_> = scaffolds
        .split(|x| *x == 10)
        .filter(|slice| !slice.is_empty())
        .collect();
    let mut res = 0;
    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            if (0..3).all(|k| map[i + k - 1][j] == 35)
                && (0..2).all(|k| map[i][j + 2 * k - 1] == 35)
            {
                res += i * j;
            }
        }
    }
    println!("{}", res);
}
