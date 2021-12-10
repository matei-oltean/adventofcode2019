use std::collections::HashMap;

type CodeType = isize;
pub type Program = HashMap<usize, CodeType>;

pub struct Machine {
    program: Program,
    index: usize,
    phase: Option<CodeType>,
    base: CodeType,
}

impl Machine {
    pub fn new(program: Program, phase: Option<CodeType>) -> Self {
        Machine {
            program,
            index: 0,
            phase,
            base: 0_isize,
        }
    }

    pub fn from_file(path: &str, phase: Option<CodeType>) -> Self {
        let program = program_from_file(path);
        Machine::new(program, phase)
    }

    pub fn process(&mut self, input: Option<CodeType>) -> Vec<CodeType> {
        let mut result = vec![];
        let mut used_input = false;
        let pow_10: Vec<CodeType> = vec![100, 1_000, 10_000];
        loop {
            let operation = self.get(self.index);
            let code = operation % 100;
            if code == 99 {
                return result;
            }
            let arg_length = number_of_args(code);

            let args: Vec<CodeType> = (0..arg_length)
                .map(|i| match operation / pow_10[(i as usize)] % 10 {
                    0 => self.get(self.index + i + 1),
                    1 => (self.index + i + 1) as CodeType,
                    2 => self.base + self.get(self.index + i + 1),
                    _ => 0,
                })
                .collect();
            self.index += arg_length + 1;
            let position = *args.last().unwrap() as usize;
            let a = self.get(args[0] as usize);
            let b = if arg_length <= 1 {
                0
            } else {
                self.get(args[1] as usize)
            };
            match code {
                1 => {
                    self.set(position, a + b);
                }
                2 => {
                    self.set(position, a * b);
                }
                3 => match input {
                    None => {
                        self.index -= 2;
                        break;
                    }
                    Some(i) => {
                        if used_input {
                            self.index -= 2;
                            break;
                        }
                        let inp = match self.phase {
                            Some(n) => {
                                self.phase = None;
                                n
                            }
                            None => {
                                used_input = true;
                                i
                            }
                        };
                        self.set(position as usize, inp);
                    }
                },
                4 => {
                    result.push(a);
                }
                5 => {
                    if a != 0 {
                        self.index = b as usize
                    }
                }
                6 => {
                    if a == 0 {
                        self.index = b as usize
                    }
                }
                7 => {
                    self.set(position as usize, (a < b) as CodeType);
                }
                8 => {
                    self.set(position as usize, (a == b) as CodeType);
                }
                9 => {
                    self.base += a;
                }
                _ => (),
            };
        }
        result
    }

    pub fn set(&mut self, index: usize, value: CodeType) {
        self.program.insert(index, value);
    }

    pub fn get(&mut self, index: usize) -> CodeType {
        match self.program.get(&index) {
            None => {
                self.set(index, 0);
                0
            }
            Some(n) => *n,
        }
    }

    pub fn print(&self) {
        dbg!(&self.program);
    }
}

pub fn program_from_file(path: &str) -> Program {
    reader::read_input(path)
        .split(',')
        .map(|c| c.parse::<CodeType>().unwrap())
        .enumerate()
        .collect()
}

fn number_of_args(code: CodeType) -> usize {
    match code {
        1 | 2 | 7 | 8 => 3,
        3 | 4 | 9 => 1,
        5 | 6 => 2,
        _ => 0,
    }
}
