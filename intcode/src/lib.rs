use std::collections::HashMap;

pub type CodeType = isize;
pub type Program = HashMap<CodeType, CodeType>;

pub struct Machine {
    program: Program,
    index: CodeType,
    phase: Option<CodeType>,
}

impl Machine {
    pub fn new(program: Program, phase: Option<CodeType>) -> Self {
        Machine {
            program: program,
            index: 0 as CodeType,
            phase: phase,
        }
    }

    pub fn from_file(path: &str, phase: Option<CodeType>) -> Self {
        let program = reader::read_input(path)
            .split(",")
            .map(|c| c.parse::<CodeType>().unwrap())
            .enumerate()
            .map(|(i, c)| (i as CodeType, c))
            .collect();
        Machine {
            program: program,
            index: 0 as CodeType,
            phase: phase,
        }
    }

    pub fn process(&mut self, input: CodeType) -> Option<CodeType> {
        let mut result = None;
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
                    0 => self.get(self.get(self.index + (i as CodeType) + 1)),
                    1 => self.get(self.index + (i as CodeType) + 1),
                    _ => 0,
                })
                .collect();
            self.index += arg_length + 1;
            let position = self.get(self.index - 1);
            match code {
                1 => {
                    self.set(position, args[0] + args[1]);
                }
                2 => {
                    self.set(position, args[0] * args[1]);
                }
                3 => {
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
                            input
                        }
                    };
                    self.set(position, inp);
                }
                4 => {
                    println!("{}", args[0]);
                    result = Some(args[0]);
                }
                5 => {
                    if args[0] != 0 {
                        self.index = args[1]
                    }
                }
                6 => {
                    if args[0] == 0 {
                        self.index = args[1]
                    }
                }
                7 => {
                    self.set(position, (args[0] < args[1]) as CodeType);
                }
                8 => {
                    self.set(position, (args[0] == args[1]) as CodeType);
                }
                _ => (),
            };
        }
        result
    }

    pub fn set(&mut self, index: CodeType, value: CodeType) {
        self.program.insert(index, value);
    }

    pub fn get(&self, index: CodeType) -> CodeType {
        *self.program.get(&index).unwrap()
    }

    pub fn print(&self) {
        dbg!(&self.program);
    }
}

fn number_of_args(code: CodeType) -> CodeType {
    match code {
        1 | 2 | 7 | 8 => 3,
        3 | 4 => 1,
        5 | 6 => 2,
        _ => 0,
    }
}
