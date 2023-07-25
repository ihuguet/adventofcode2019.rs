use std::collections::VecDeque;
use itertools::Itertools;

type Prog = Vec<i64>;
type SolveFn = dyn Fn(&Prog, &[i64; 5]) -> Option<i64>;

struct Computer {
    prog: Prog,
    ip: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    halted: bool
}

#[derive(PartialEq)]
enum ComputerState {
    Halt,
    WaitInput
}

enum OpArg {
    Pos(i64),
    Imm(i64)
}

fn main() {
    let mut lines = aoc::input::parse_tokens_split_str_unsafe::<i64>("day07", ",");
    let prog: Prog = lines.next().unwrap();

    let result = run_with_permutations(&prog, [0, 1, 2, 3, 4], &solve_part1);
    println!("Part 1: result = {}", result);

    let result = run_with_permutations(&prog, [5, 6, 7, 8, 9], &solve_part2);
    println!("Part 2: result = {}", result);
}

fn run_with_permutations(prog: &Prog, inputs: [i64; 5], solve_fn: &SolveFn) -> i64 {
    let mut max_val = 0;

    for permutation in inputs.into_iter().permutations(inputs.len()) {
        let permutation: [i64; 5] = permutation.try_into().unwrap();

        let val = solve_fn(prog, &permutation);
        if let Some(val) = val {
            max_val = max_val.max(val);
        }
    }

    max_val
}

fn solve_part1(prog: &Prog, inputs: &[i64; 5]) -> Option<i64> {
    let mut val = 0;

    for i in 0..5 {
        let mut computer = Computer::new(prog.clone());
        computer.push_input(inputs[i]);
        computer.push_input(val);

        computer.run().ok()?;
        val = computer.pop_output()?;

        computer.output.clear();
    }

    Some(val)
}

fn solve_part2(prog: &Prog, inputs: &[i64; 5]) -> Option<i64> {
    let mut computers: Vec<Computer> = inputs.iter()
        .map(|input| {
            let mut computer = Computer::new(prog.clone());
            computer.push_input(*input);
            computer
        }).collect();

    let mut io_pipe = VecDeque::from([0]);

    loop {
        for i in (0..5).cycle() {
            if computers[i].halted {
                return None;
            }

            computers[i].extend_input(io_pipe.drain(..));
            let status = computers[i].run().ok()?;
            io_pipe.extend(computers[i].drain_output());

            if i == 4 && status == ComputerState::Halt {
                return match io_pipe.len() {
                    1 => Some(io_pipe[0]),
                    _ => None
                };
            }
        }
    }
}

impl Computer {
    fn new(prog: Vec<i64>) -> Computer {
        Computer {
            prog,
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            halted: false
        }
    }

    fn run(self: &mut Computer) -> Result<ComputerState, i64> {
        return loop {
            let op_and_mode = self.get(OpArg::Imm(self.ip))?;
            let op = op_and_mode % 100;

            let mode_arg1 = (op_and_mode / 100) % 10;
            let arg1 = OpArg::new(mode_arg1, self.ip + 1).map_err(|_| self.ip)?;

            let mode_arg2 = (op_and_mode / 1000) % 10;
            let arg2 = OpArg::new(mode_arg2, self.ip + 2).map_err(|_| self.ip)?;

            let mode_arg3 = (op_and_mode / 10000) % 10;
            let arg3 = OpArg::new(mode_arg3, self.ip + 3).map_err(|_| self.ip)?;

            match op {
                1 => { // ADD
                    self.set(arg3, self.get(arg1)? + self.get(arg2)?)?;
                    self.ip += 4;
                },
                2 => { // MUL
                    self.set(arg3, self.get(arg1)? * self.get(arg2)?)?;
                    self.ip += 4;
                },
                3 => { // READ
                    match self.pop_input() {
                        Some(val) => {
                            self.set(arg1, val)?;
                            self.ip += 2;
                        },
                        None => {
                            return Ok(ComputerState::WaitInput);
                        }
                    }
                },
                4 => { // WRITE
                    assert_eq!(op_and_mode / 1000, 0);
                    self.push_output(self.get(arg1)?);
                    self.ip += 2;
                },
                5 => { // JUMP IF TRUE
                    if self.get(arg1)? != 0 {
                        self.ip = self.get(arg2)?;
                    } else {
                        self.ip += 3;
                    }
                },
                6 => { // JUMP IF FALSE
                    if self.get(arg1)? == 0 {
                        self.ip = self.get(arg2)?;
                    } else {
                        self.ip += 3;
                    }
                },
                7 => { // LESS THAN
                    if self.get(arg1)? < self.get(arg2)? {
                        self.set(arg3, 1)?;
                    } else {
                        self.set(arg3, 0)?;
                    }
                    self.ip += 4;
                },
                8 => { // EQUAL
                    if self.get(arg1)? == self.get(arg2)? {
                        self.set(arg3, 1)?;
                    } else {
                        self.set(arg3, 0)?;
                    }
                    self.ip += 4;
                }
                99 => { // HALT
                    self.halted = true;
                    break Ok(ComputerState::Halt);
                },
                _ => {
                    break Err(self.ip);
                }
            } // match op
        } // loop
    }

    fn push_input(&mut self, val: i64) {
        self.input.push_front(val);
    }

    fn extend_input(&mut self, vals: impl Iterator<Item = i64>) {
        for val in vals {
            self.push_input(val);
        }
    }

    fn pop_input(&mut self) -> Option<i64> {
        self.input.pop_back()
    }

    fn push_output(&mut self, val: i64) {
        self.output.push_front(val);
    }

    fn pop_output(&mut self) -> Option<i64> {
        self.output.pop_back()
    }

    fn drain_output(&mut self) -> impl IntoIterator<Item = i64> + '_ {
        self.output.drain(..).rev()
    }

    fn get(&self, arg: OpArg) -> Result<i64, i64> {
        match arg {
            OpArg::Pos(pos) => {
                let addr = *self.prog.get(pos as usize).ok_or(pos)?;
                Ok(*self.prog.get(addr as usize).ok_or(pos)?)
            },
            OpArg::Imm(pos) => {
                Ok(*self.prog.get(pos as usize).ok_or(pos)?)
            }
        }
    }

    fn set(&mut self, arg: OpArg, val: i64) -> Result<(), i64> {
        match arg {
            OpArg::Imm(_) => {
                panic!("Unexpected immediate arg");
            },
            OpArg::Pos(pos) => {
                let addr = *self.prog.get(pos as usize).ok_or(pos)?;
                *self.prog.get_mut(addr as usize).ok_or(pos)? = val;
            }
        }
        Ok(())
    }
}

impl OpArg {
    fn new(arg_mode: i64, prog_pos: i64) -> Result<OpArg, i64> {
        match arg_mode {
            0 => Ok(OpArg::Pos(prog_pos)),
            1 => Ok(OpArg::Imm(prog_pos)),
            _ => Err(arg_mode)
        }
    }
}
