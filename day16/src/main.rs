fn get_pattern(index: usize, element: usize) -> i8 {
    let base_pattern = [0, 1, 0, -1];
    let pattern_size = (element + 1) * base_pattern.len();
    base_pattern[((index + 1) % pattern_size) / (element + 1)]
}

fn take(number: usize, array: &[usize]) -> usize {
    array
        .iter()
        .take(number)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn do_iteration(iterations: usize, array: &mut Vec<usize>) {
    let n = array.len();
    for _ in 0..iterations {
        let new_array: Vec<usize> = (0..n)
            .map(|i| {
                (array
                    .iter()
                    .enumerate()
                    .skip(i)
                    .map(|(j, ar)| *ar as isize * get_pattern(j, i) as isize)
                    .sum::<isize>()
                    .abs()
                    % 10) as usize
            })
            .collect();
        *array = new_array;
    }
}

fn do_iteration_with_offset(iterations: usize, array: &[usize], repeat: usize) -> usize {
    let n = array.len();
    let offset = take(7, &array);
    let mut array_res: Vec<usize> = (offset..repeat * n)
        .map(|i| array[i % n] as usize)
        .collect();
    for _ in 0..iterations {
        let mut temp = array_res.iter().sum::<usize>();
        for i in 0..array_res.len() {
            let t = array_res[i];
            array_res[i] = temp % 10;
            temp -= t;
        }
    }
    take(8, &array_res)
}

fn main() {
    let mut input: Vec<_> = reader::read_input("16.input")
        .chars()
        .map(|s| s.to_digit(10).unwrap() as usize)
        .collect();
    //do_iteration(100, &mut input);
    //println!("{}", take(8, &input));
    println!("{}", do_iteration_with_offset(100, &mut input, 10_000));
}
