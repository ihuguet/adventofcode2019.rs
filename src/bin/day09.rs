use std::collections::{VecDeque, HashMap};

type Mem = HashMap<usize, i64>;

struct Prog {
    mem: Mem,
    ip: i64,
    rel_base: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    halted: bool
}

#[derive(PartialEq)]
enum ProgState {
    Halt,
    WaitInput
}

enum Arg {
    Pos(i64),
    Imm(i64),
    Rel(i64),
}

fn main() {
    let mut lines = aoc::input::parse_tokens_split_str_unsafe::<i64>("day09", ",");
    let mem: Mem = lines.next().unwrap()
        .into_iter()
        .enumerate()
        .collect();

    let result = solve(mem.clone(), 1);
    println!("Part 1: result = {}", result);

    let result = solve(mem, 2);
    println!("Part 2: result = {}", result);
}

fn solve(mem: Mem, input: i64) -> i64 {
    let mut prog = Prog::new(mem);
    prog.push_input(input);
    prog.run().expect("Program error");
    assert_eq!(prog.output.len(), 1);
    prog.pop_output().unwrap()
}


impl Prog {
    fn new(mem: Mem) -> Prog {
        Prog {
            mem,
            ip: 0,
            rel_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            halted: false
        }
    }

    fn run(self: &mut Prog) -> Result<ProgState, i64> {
        return loop {
            let op_and_mode = self.mem_get(Arg::Imm(self.ip))?;
            let op = op_and_mode % 100;

            let mode_arg1 = (op_and_mode / 100) % 10;
            let arg1 = Arg::new(mode_arg1, self.ip + 1).map_err(|_| self.ip)?;

            let mode_arg2 = (op_and_mode / 1000) % 10;
            let arg2 = Arg::new(mode_arg2, self.ip + 2).map_err(|_| self.ip)?;

            let mode_arg3 = (op_and_mode / 10000) % 10;
            let arg3 = Arg::new(mode_arg3, self.ip + 3).map_err(|_| self.ip)?;

            match op {
                1 => { // ADD
                    self.mem_set(arg3, self.mem_get(arg1)? + self.mem_get(arg2)?)?;
                    self.ip += 4;
                },
                2 => { // MUL
                    self.mem_set(arg3, self.mem_get(arg1)? * self.mem_get(arg2)?)?;
                    self.ip += 4;
                },
                3 => { // READ
                    match self.pop_input() {
                        Some(val) => {
                            self.mem_set(arg1, val)?;
                            self.ip += 2;
                        },
                        None => {
                            return Ok(ProgState::WaitInput);
                        }
                    }
                },
                4 => { // WRITE
                    assert_eq!(op_and_mode / 1000, 0);
                    self.push_output(self.mem_get(arg1)?);
                    self.ip += 2;
                },
                5 => { // JUMP IF TRUE
                    if self.mem_get(arg1)? != 0 {
                        self.ip = self.mem_get(arg2)?;
                    } else {
                        self.ip += 3;
                    }
                },
                6 => { // JUMP IF FALSE
                    if self.mem_get(arg1)? == 0 {
                        self.ip = self.mem_get(arg2)?;
                    } else {
                        self.ip += 3;
                    }
                },
                7 => { // LESS THAN
                    if self.mem_get(arg1)? < self.mem_get(arg2)? {
                        self.mem_set(arg3, 1)?;
                    } else {
                        self.mem_set(arg3, 0)?;
                    }
                    self.ip += 4;
                },
                8 => { // EQUAL
                    if self.mem_get(arg1)? == self.mem_get(arg2)? {
                        self.mem_set(arg3, 1)?;
                    } else {
                        self.mem_set(arg3, 0)?;
                    }
                    self.ip += 4;
                },
                9 => { // REL BASE
                    self.rel_base += self.mem_get(arg1)?;
                    self.ip += 2;
                },
                99 => { // HALT
                    self.halted = true;
                    break Ok(ProgState::Halt);
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

    // unused
    fn _extend_input(&mut self, vals: impl Iterator<Item = i64>) {
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

    // unused
    fn _drain_output(&mut self) -> impl IntoIterator<Item = i64> + '_ {
        self.output.drain(..).rev()
    }

    fn mem_get(&self, arg: Arg) -> Result<i64, i64> {
        match arg {
            Arg::Pos(pos) => self._mem_get_pos(pos, 0),
            Arg::Imm(pos) => self._mem_get_imm(pos),
            Arg::Rel(pos) => self._mem_get_pos(pos, self.rel_base),
        }
    }

    fn _mem_get_pos(&self, pos: i64, pos_base: i64) -> Result<i64, i64> {
        match pos {
            0.. => self._mem_get_imm(
                pos_base + *self.mem.get(&(pos as usize)).unwrap_or(&0)
            ),
            _   => Err(pos),
        }
    }

    fn _mem_get_imm(&self, pos: i64) -> Result<i64, i64> {
        match pos {
            0.. => Ok(*self.mem.get(&(pos as usize)).unwrap_or(&0)),
            _   => Err(pos),
        }
    }

    fn mem_set(&mut self, arg: Arg, val: i64) -> Result<(), i64> {
        match arg {
            Arg::Imm(pos) => Err(pos),
            Arg::Pos(pos) => self._mem_set_pos(pos, 0, val),
            Arg::Rel(pos) => self._mem_set_pos(pos, self.rel_base, val),
        }
    }

    fn _mem_set_pos(&mut self, pos: i64, pos_base: i64, val: i64) -> Result<(), i64> {
        let addr = match pos {
            0.. => pos_base + *self.mem.get(&(pos as usize)).unwrap_or(&0),
            _   => return Err(pos),
        };
        match addr {
            0.. => self.mem.insert(addr as usize, val),
            _   => return Err(pos),
        };
        Ok(())
    }
}

impl Arg {
    fn new(arg_mode: i64, mem_pos: i64) -> Result<Arg, i64> {
        match arg_mode {
            0 => Ok(Arg::Pos(mem_pos)),
            1 => Ok(Arg::Imm(mem_pos)),
            2 => Ok(Arg::Rel(mem_pos)),
            _ => Err(arg_mode)
        }
    }
}
