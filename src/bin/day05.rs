use std::collections::VecDeque;

struct Prog(Vec<i64>);

enum OpArg {
    Pos(i64),
    Imm(i64)
}

fn main() {
    let mut lines = aoc::input::parse_tokens_split_str_unsafe::<i64>("day05", ",");
    let prog= lines.next().unwrap();

    let result = *run_program(Prog(prog.clone()), &[1])
        .expect("Program error")
        .last()
        .expect("Output is empty");
    println!("Part 1: diagnostic code = {}", result);

    let result = *run_program(Prog(prog), &[5])
        .expect("Program error")
        .last()
        .expect("Output is empty");
    println!("Part 2: diagnostic code = {}", result);
}

fn run_program(mut prog: Prog, input: &[i64]) -> Result<Vec<i64>, i64> {
    let mut pos = 0;
    let mut input = VecDeque::from_iter(input.iter());
    let mut output = Vec::new();

    return loop {
        let op_and_mode = prog.get(OpArg::Imm(pos))?;
        let op = op_and_mode % 100;

        let mode_arg1 = (op_and_mode / 100) % 10;
        let arg1 = OpArg::new(mode_arg1, pos + 1).map_err(|_| pos)?;

        let mode_arg2 = (op_and_mode / 1000) % 10;
        let arg2 = OpArg::new(mode_arg2, pos + 2).map_err(|_| pos)?;

        let mode_arg3 = (op_and_mode / 10000) % 10;
        let arg3 = OpArg::new(mode_arg3, pos + 3).map_err(|_| pos)?;

        match op {
            1 => { // ADD
                prog.set(arg3, prog.get(arg1)? + prog.get(arg2)?)?;
                pos += 4;
            },
            2 => { // MUL
                prog.set(arg3, prog.get(arg1)? * prog.get(arg2)?)?;
                pos += 4;
            },
            3 => { // READ
                prog.set(arg1, *input.pop_front().ok_or(-1)?)?;
                pos += 2;
            },
            4 => { // WRITE
                assert_eq!(op_and_mode / 1000, 0);
                output.push(prog.get(arg1)?);
                pos += 2;
            },
            5 => { // JUMP IF TRUE
                if prog.get(arg1)? != 0 {
                    pos = prog.get(arg2)?;
                } else {
                    pos += 3;
                }
            },
            6 => { // JUMP IF FALSE
                if prog.get(arg1)? == 0 {
                    pos = prog.get(arg2)?;
                } else {
                    pos += 3;
                }
            },
            7 => { // LESS THAN
                if prog.get(arg1)? < prog.get(arg2)? {
                    prog.set(arg3, 1)?;
                } else {
                    prog.set(arg3, 0)?;
                }
                pos += 4;
            },
            8 => { // EQUAL
                if prog.get(arg1)? == prog.get(arg2)? {
                    prog.set(arg3, 1)?;
                } else {
                    prog.set(arg3, 0)?;
                }
                pos += 4;
            }
            99 => { // HALT
                break Ok(output);
            },
            _ => {
                break Err(pos);
            }
        } // match op
    } // loop
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

impl Prog {
    fn get(&self, arg: OpArg) -> Result<i64, i64> {
        match arg {
            OpArg::Pos(pos) => {
                let addr = *self.0.get(pos as usize).ok_or(pos)?;
                Ok(*self.0.get(addr as usize).ok_or(pos)?)
            },
            OpArg::Imm(pos) => {
                Ok(*self.0.get(pos as usize).ok_or(pos)?)
            }
        }
    }

    fn set(&mut self, arg: OpArg, val: i64) -> Result<(), i64> {
        match arg {
            OpArg::Imm(_) => {
                panic!("Unexpected immediate arg");
            },
            OpArg::Pos(pos) => {
                let addr = *self.0.get(pos as usize).ok_or(pos)?;
                *self.0.get_mut(addr as usize).ok_or(pos)? = val;
            }
        }
        Ok(())
    }
}
