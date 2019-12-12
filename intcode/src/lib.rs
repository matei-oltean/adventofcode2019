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

    pub fn process(&mut self, input: CodeType) -> Option<CodeType> {
        let mut result = None;
        let mut used_input = false;
        let (pow_10, _) = (0..3).fold((Vec::new(), 100), |(mut pow_10, pow), _| {
            pow_10.push(pow);
            (pow_10, pow * 10)
        });
        loop {
            let operation = self.get(self.index);
            let code = operation % 100;
            if code == 99 {
                return result;
            }
            let arg_length = number_of_args(code);

            let args: Vec<CodeType> = (0..arg_length)
                .map(| i | match operation / pow_10[(i as usize)] % 10 {
                    0 => self.get(self.get(self.index + (i as CodeType) + 1)),
                    1 => self.get(self.index + (i as CodeType) + 1),
                    _ => 0,
                })
                .collect();
            self.index += arg_length + 1;
            let position = self.get(self.index - 1);
            let next_result = match code {
                1 => {
                    self.set(position, args[0] + args[1]);
                    None
                }
                2 => {
                    self.set(position, args[0] * args[1]);
                    None
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
                    None
                }
                4 => {
                    println!("{}", args[0]);
                    Some(args[0])
                }
                5 => {
                    if args[0] != 0 {
                        self.index = args[1]
                    }
                    None
                }
                6 => {
                    if args[0] == 0 {
                        self.index = args[1]
                    }
                    None
                }
                7 => {
                    self.set(position, (args[0] < args[1]) as CodeType);
                    None
                }
                8 => {
                    self.set(position, (args[0] == args[1]) as CodeType);
                    None
                }
                _ => None,
            };
            if next_result != None {
                result = next_result;
            }
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
