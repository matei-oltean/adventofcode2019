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
    let mut l = 0;
    for (i, val) in scaffolds.iter_mut().enumerate() {
        if val == &10 {
            l = i;
            break;
        }
    }
    let map_1d: Vec<_> = scaffolds
        .iter()
        .filter(|x| **x as usize != 10)
        .collect::<Vec<_>>();
    let map: Vec<_> = map_1d.chunks(l).collect();
    let mut res = 0;
    for i in 1..map.len() - 1 {
        for j in 1..l - 1 {
            if (0..3).all(|k| map[i + k - 1][j] == &35)
                && (0..2).all(|k| map[i][j + 2 * k - 1] == &35)
            {
                res += i * j;
            }
        }
    }
    println!("{}", res);
}
