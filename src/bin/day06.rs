use aoc;
use std::collections::HashMap;

fn main() {
    let lines = aoc::input::parse_tokens_split_str_unsafe::<String>("day06", ")");
    let orbits = lines
        .map(|tokens| (tokens[1].clone(), tokens[0].clone()))
        .collect::<HashMap<_, _>>();

    let orbits_count = part1(&orbits);
    println!("Part 1: orbits count = {}", orbits_count);

    let orbits_count = part2(&orbits);
    println!("Part 2: orbit transfers = {}", orbits_count);
}

fn part1(orbits: &HashMap<String, String>) -> u32 {
    let mut orbits_count = 0;

    for mut sat in orbits.keys() {
        while sat != "COM" {
            orbits_count += 1;
            sat = orbits.get(sat).expect(&format!("Object {} not found", sat));
        }
    }

    orbits_count
}

fn part2(orbits: &HashMap<String, String>) -> u32 {
    let orig = orbits.get("YOU").expect("Object YOU not found");
    let dest = orbits.get("SAN").expect("Object SAN not found");

    if orig == dest {
        return 0;
    }

    let mut sat = orig;
    let mut path1 = Vec::new();
    while sat != "COM" {
        sat = orbits.get(sat).unwrap();
        path1.push(sat);
    }

    sat = dest;
    let mut path2_len = 0;
    while let None = path1.iter().position(|item| item == &sat) {
        path2_len += 1;
        sat = orbits.get(sat).unwrap();
    }

    let path1_len = path1.iter().position(|item| item == &sat).unwrap() + 1;
    path1_len as u32 + path2_len
}
