use std::collections::HashMap;

pub type CodeType = isize;
pub type Program = HashMap<CodeType, CodeType>;

pub struct Machine {
    program: Program,
    index: CodeType,
    phase: Option<CodeType>,
    base: CodeType,
}

impl Machine {
    pub fn new(program: Program, phase: Option<CodeType>) -> Self {
        Machine {
            program: program,
            index: 0 as CodeType,
            phase: phase,
            base: 0 as CodeType,
        }
    }

    pub fn from_file(path: &str, phase: Option<CodeType>) -> Self {
        let program = reader::read_input(path)
            .split(",")
            .map(|c| c.parse::<CodeType>().unwrap())
            .enumerate()
            .map(|(i, c)| (i as CodeType, c))
            .collect();
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
                    0 => self.get(self.index + (i as CodeType) + 1),
                    1 => self.index + (i as CodeType) + 1,
                    2 => self.base + self.get(self.index + (i as CodeType) + 1),
                    _ => 0,
                })
                .collect();
            self.index += arg_length + 1;
            let position = *args.last().unwrap();
            let a = self.get(args[0]);
            let b = if arg_length <= 1 {
                0
            } else {
                self.get(args[1])
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
                        self.set(position, inp);
                    }
                },
                4 => {
                    result.push(a);
                }
                5 => {
                    if a != 0 {
                        self.index = b
                    }
                }
                6 => {
                    if a == 0 {
                        self.index = b
                    }
                }
                7 => {
                    self.set(position, (a < b) as CodeType);
                }
                8 => {
                    self.set(position, (a == b) as CodeType);
                }
                9 => {
                    self.base += a;
                }
                _ => (),
            };
        }
        result
    }

    pub fn set(&mut self, index: CodeType, value: CodeType) {
        self.program.insert(index, value);
    }

    pub fn get(&mut self, index: CodeType) -> CodeType {
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

fn number_of_args(code: CodeType) -> CodeType {
    match code {
        1 | 2 | 7 | 8 => 3,
        3 | 4 | 9 => 1,
        5 | 6 => 2,
        _ => 0,
    }
}
