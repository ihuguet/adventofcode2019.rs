use aoc;

type Coord = (i32, i32);

#[derive(Clone)]
struct AsteroidMap {
    asteroids: Vec<Coord>,
    size: (i32, i32)
}

fn main() {
    let asteroids_map = parse_input();

    let (station, detected_asteroids) = asteroids_map.get_best_station();
    println!("Part 1: max visible asteroids = {}", detected_asteroids.len());

    let last_destroyed = asteroids_map.get_nth_destroyed_from(station, 200);
    println!("Part 2: 200th destroyed asteroid = {}", last_destroyed.0 * 100 + last_destroyed.1);
}

impl AsteroidMap {
    fn get_best_station(&self) -> (Coord, Vec<Coord>) {
        self.asteroids.iter().copied()
            .map(|a| (a, self.get_detected_asteroids_from(a).0))
            .max_by_key(|(_, visible_asteroids)| visible_asteroids.len() as u32)
            .unwrap()
    }

    fn get_nth_destroyed_from(&self, orig: Coord, nth: usize) -> Coord {
        let mut count = 0;
        let mut map = self.clone();
        map.asteroids.retain(|a| *a != orig);

        while !map.asteroids.is_empty() {
            // get next round targets and order by angle
            let (mut detected, undetected) = map.get_detected_asteroids_from(orig);
            detected.sort_by(|a, b| {
                let a = ((a.0 - orig.0) as f64).atan2((a.1 - orig.1) as f64);
                let b = ((b.0 - orig.0) as f64).atan2((b.1 - orig.1) as f64);
                b.partial_cmp(&a).unwrap()
            });

            // if the nth target is in this round return it
            if count + detected.len() >= nth {
                return detected[nth - count - 1];
            }

            // else, prepare for next round
            count += detected.len();
            map.asteroids = undetected;
        }

        (-1, -1)
    }

    fn get_detected_asteroids_from(&self, orig: Coord) -> (Vec<Coord>, Vec<Coord>) {
        let mut asteroids = self.asteroids.clone();
        asteroids.retain(|a| *a != orig);
        let mut detected = Vec::with_capacity(asteroids.len());
        let mut undetected = Vec::with_capacity(asteroids.len());

        // evaluate closest asteroids first
        asteroids.sort_by_key(|a| -manhattan_dist(orig, *a));

        while let Some(mut asteroid) = asteroids.pop() {
            detected.push(asteroid);

            // remove asteroids shadowed by the current one
            let mut diff = (asteroid.0 - orig.0, asteroid.1 - orig.1);
            let div = gcd(diff.0, diff.1);
            diff = (diff.0 / div, diff.1 / div);

            loop {
                asteroid = (asteroid.0 + diff.0, asteroid.1 + diff.1);

                if !self.is_inside(asteroid) {
                    break;
                } else if let Some(i) = asteroids.iter().position(|a| *a == asteroid) {
                    asteroids.remove(i);
                    undetected.push(asteroid);
                }
            }
        }

        (detected, undetected)
    }

    fn is_inside(&self, point: Coord) -> bool {
        (0..self.size.0).contains(&point.0) && (0..self.size.1).contains(&point.1)
    }
}

fn manhattan_dist(a: Coord, b: Coord) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_input() -> AsteroidMap {
    parse_lines(aoc::input::read_lines("day10"))
}

fn parse_lines(lines: impl Iterator<Item = String>) -> AsteroidMap {
    let mut asteroids = Vec::new();
    let (mut x, mut y) = (0, 0);

    for line in lines {
        asteroids.extend(
            line.match_indices("#").map(|(x, _)| (x as i32, y))
        );
        x = line.len() as i32;
        y += 1;
    }

    AsteroidMap { asteroids, size: (x, y) }
}

fn gcd(first: i32, second: i32) -> i32 {
    match (first, second) {
        (0, 0) => return 1,
        (0, v) => return v.abs(),
        (v, 0) => return v.abs(),
        _ => (),
    }

    let mut max = first.abs().max(second.abs());
    let mut min = first.abs().min(second.abs());

    loop {
        match max % min {
            0 => return min,
            res => {
                max = min;
                min = res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP_1: &str =
        ".#..#\n\
         .....\n\
         #####\n\
         ....#\n\
         ...##";

    const MAP_2: &str =
        "......#.#.\n\
         #..#.#....\n\
         ..#######.\n\
         .#.#.###..\n\
         .#..#.....\n\
         ..#....#.#\n\
         #..#....#.\n\
         .##.#..###\n\
         ##...#..#.\n\
         .#....####";

    const MAP_3: &str =
        ".#....#####...#..\n\
         ##...##.#####..##\n\
         ##...#...#.#####.\n\
         ..#.....X...###..\n\
         ..#.#.....#....##";

    const MAP_4: &str =
        ".#..##.###...#######\n\
         ##.############..##.\n\
         .#.######.########.#\n\
         .###.#######.####.#.\n\
         #####.##.#.##.###.##\n\
         ..#####..#.#########\n\
         ####################\n\
         #.####....###.#.#.##\n\
         ##.#################\n\
         #####.##.###..####..\n\
         ..######..##.#######\n\
         ####.##.####...##..#\n\
         .#####..#.######.###\n\
         ##...#.##########...\n\
         #.##########.#######\n\
         .####.#.###.###.#.##\n\
         ....##.##.###..#####\n\
         .#.#.###########.###\n\
         #.#.#.#####.####.###\n\
         ###.##.####.##.#..##";

    fn test_count_from(map: &str, orig: Coord, expect: u32) {
        let asteroid_map = parse_lines(map.lines().map(String::from));
        let result = asteroid_map.get_detected_asteroids_from(orig).0.len() as u32;
        assert_eq!(result, expect, "Expected {}, got {}", expect, result);
    }

    fn test_count_max(map: &str, expect: u32) {
        let asteroid_map = parse_lines(map.lines().map(String::from));
        let (_, visible_asteroids) = asteroid_map.get_best_station();
        let result = visible_asteroids.len() as u32;
        assert_eq!(result, expect, "Expected {}, got {}", expect, result);
    }

    fn test_destroy(map: &str, orig: Coord, nth: usize, expect: i32) {
        let asteroid_map = parse_lines(map.lines().map(String::from));
        let result = asteroid_map.get_nth_destroyed_from(orig, nth);
        let result = result.0 * 100 + result.1;
        assert_eq!(result, expect, "Expected {}, got {}", expect, result);
    }

    #[test]
    fn test_count_from_1() {
         test_count_from(MAP_1, (3, 4), 8);
    }

    #[test]
    fn test_count_from_2() {
        test_count_from(MAP_2, (5, 8), 33);
    }

    #[test]
    fn test_count_max_1() {
        test_count_max(MAP_1, 8);
    }

    #[test]
    fn test_count_max_2() {
        test_count_max(MAP_2, 33);
    }

    #[test]
    fn test_destroy_3() {
        test_destroy(MAP_3, (8, 3), 1, 801);
        test_destroy(MAP_3, (8, 3), 2, 900);
        test_destroy(MAP_3, (8, 3), 3, 901);
        test_destroy(MAP_3, (8, 3), 4, 1000);
    }

    #[test]
    fn test_destroy_4() {
        test_destroy(MAP_4, (11, 13), 200, 802);
    }
}
