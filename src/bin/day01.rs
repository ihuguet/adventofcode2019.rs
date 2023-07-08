use aoc;

fn main() {
    let lines = aoc::input::read_lines("day01");

    for line in lines {
        print!("{}", line);
    }
    println!();
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        assert!(1 == 1);
    }
}
