use aoc;

fn main() {
    let lines = aoc::input::parse_lines_unsafe::<i32>("day01");

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for val in lines {
        total_part1 += val / 3 - 2;
        total_part2 += calc_recursive(val);
    }

    println!("Part 1: total fuel = {}", total_part1);
    println!("Part 2: total fuel = {}", total_part2);
}

fn calc_recursive(mut val: i32) -> i32 {
    val = val / 3 - 2;
    if val >= 0 {
        return val + calc_recursive(val);
    } else {
        return 0;
    }
}