use aoc;

fn main() {
    let mut lines = aoc::input::parse_tokens_split_str_unsafe::<u64>("day02", ",");
    let values = lines.next().unwrap();

    let result = part1(values.clone());
    println!("Part 1: pos0 = {}", result);

    let result = part2(&values);
    println!("Part 2: result = {}", result);
}

fn part1(mut values: Vec<u64>) -> u64 {
    values[1] = 12;
    values[2] = 2;
    return solve(&mut values).unwrap();
}

fn part2(original_values: &[u64]) -> u64 {
    let mut min = 0;
    let mut max = 32;

    loop {
        for val1 in min..max {
            for val2 in 0..max {
                let mut values = original_values.to_vec();
                values[1] = val1;
                values[2] = val2;

                if let Ok(result) = solve(&mut values) {
                    if result == 19690720 {
                        return 100 * val1 + val2;
                    }
                }
            }
        }

        min = max;
        max *= 2;
    }
}

const OP_ADD: u64 = 1;
const OP_MUL: u64 = 2;
const OP_HALT: u64 = 99;

fn solve(values: &mut [u64]) -> Result<u64, usize> {
    let mut pos: usize = 0;

    return loop {
        let op = *values.get_result(pos)?;

        if op == OP_HALT {
            break Ok(values[0]);
        }

        let input1_idx = *values.get_result(pos + 1)? as usize;
        let input2_idx = *values.get_result(pos + 2)? as usize;
        let dest_idx = *values.get_result(pos + 3)? as usize;

        let result = match op {
            OP_ADD => *values.get_result(input1_idx)? + *values.get_result(input2_idx)?,
            OP_MUL => *values.get_result(input1_idx)? * *values.get_result(input2_idx)?,
            _ => panic!("Unexpected operand '{}' at pos '{}", op, pos),
        };
        *values.get_result(dest_idx)? = result;

        pos += 4;
    }
}

trait GetResult {
    type Item: Sized;
    fn get_result(&mut self, pos: usize) -> Result<&mut Self::Item, usize>;
}

impl<T> GetResult for [T] {
    type Item = T;
    fn get_result(&mut self, pos: usize) -> Result<&mut T, usize>{
        self.get_mut(pos).ok_or(pos)
    }
}