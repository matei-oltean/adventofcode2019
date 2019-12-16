fn get_pattern(index: usize, element: usize) -> isize {
    let base_pattern = [0, 1, 0, -1];
    let pattern_size = (element + 1) * base_pattern.len();
    base_pattern[((index + 1) % pattern_size) / (element + 1)]
}

fn do_iteration(iterations: usize, array: &mut Vec<usize>) {
    let n = array.len();
    dbg!(&n);
    for _ in 0..iterations {
        let new_array: Vec<usize> = (0..n)
            .map(|i| (array.iter().enumerate()
                .map(|(j,ar)| *ar as isize * get_pattern(j, i))
                .sum::<isize>().abs() % 10) as usize)
            .collect();
        *array = new_array;
    }
}

fn main() {
    let mut input: Vec<_> = reader::read_input("16.input")
    .chars()
    .map(|s| s.to_digit(10).unwrap() as usize)
    .collect();
    do_iteration(100, &mut input);
    println!("{}", input.iter().take(8).map(|n| n.to_string()).collect::<Vec<_>>().join(""));
}
