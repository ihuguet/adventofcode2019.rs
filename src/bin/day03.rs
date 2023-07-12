use aoc;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord(isize, isize);

struct Move {
    dist: isize,
    dir: Coord
}

fn main() {
    let mut lines = aoc::input::parse_tokens_split_str_unsafe::<Move>("day03", ",");
    let moves_1 = lines.next().unwrap();
    let moves_2 = lines.next().unwrap();

    let (coords_1, distances_1) = get_visited_points(&moves_1);
    let (coords_2, distances_2) = get_visited_points(&moves_2);

    let (dists_to_center, dists_traveled): (Vec<_>, Vec<_>) = coords_1.intersection(&coords_2)
        .filter(|p| **p != Coord(0, 0))
        .map(|p| (p.0.abs() + p.1.abs(), distances_1[p] + distances_2[p]))
        .unzip();

    println!("Part 1: min dist = {}", dists_to_center.iter().min().unwrap());
    println!("Part 2: min steps = {}", dists_traveled.iter().min().unwrap());
}

fn get_visited_points(moves: &[Move]) -> (HashSet<Coord>, HashMap<Coord, u32>) {
    let mut coords = HashSet::new();
    let mut distances = HashMap::new();
    let mut pos = Coord(0, 0);
    let mut steps = 0;

    for mov in moves {
        let Move {mut dist, dir} = mov;
        while dist > 0 {
            pos = Coord(pos.0 + dir.0, pos.1 + dir.1);
            steps += 1;
            coords.insert(pos);
            distances.entry(pos).or_insert(steps);
            dist -= 1;
        }
    }
    
    (coords, distances)
}

impl FromStr for Move {
    type Err = aoc::input::ParseAoCInputError<Self>;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let dist = s[1..].parse::<isize>()?;
        let dir = match &s[..1] {
            "U" => Coord(0, 1),
            "R" => Coord(1, 0),
            "D" => Coord(0, -1),
            "L" => Coord(-1, 0),
            v => return Err(aoc::input::ParseAoCInputError::new(&format!("Unknown dir {}", v))),
        };
        Ok(Move {dist, dir})
    }
}