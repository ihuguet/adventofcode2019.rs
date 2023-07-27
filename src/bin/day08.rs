use aoc;

const COLS: usize = 25;
const ROWS: usize = 6;
const LAYER_SIZE: usize = ROWS * COLS;

//const BLACK: char = '0';
const WHITE: char = '1';
const TRANSPARENT: char = '2';

fn main() {
    let pixels = aoc::input::read_lines("day08").next().unwrap();

    let result = part1(&pixels);
    println!("Part 1: result = {}", result);

    let image = part2(&pixels);
    println!("Part 2: message:");
    print_image(&image);
}

fn part1(pixels: &str) -> u32 {
    let mut min_zeros = u32::MAX;
    let mut result = 0;

    let mut pos = 0;
    let mut counts = [0; 3];

    for pixel in pixels.chars() {
        pos += 1;

        match pixel {
            '0' => counts[0] += 1,
            '1' => counts[1] += 1,
            '2' => counts[2] += 1,
            _ => (),
        }

        if pos == LAYER_SIZE {
            if counts[0] < min_zeros {
                min_zeros = counts[0];
                result = counts[1] * counts[2];
            }
            pos = 0;
            counts.fill(0);
        }
    }

    result
}

fn part2(pixels: &str) -> [char; LAYER_SIZE] {
    let mut image = [TRANSPARENT; LAYER_SIZE];
    let mut i = 0;

    for pixel in pixels.chars() {
        if pixel != TRANSPARENT && image[i] == TRANSPARENT {
            image[i] = pixel;
        }

        i += 1;
        if i == LAYER_SIZE {
            i = 0;
        }
    }

    image
}

fn print_image(image: &[char; LAYER_SIZE]) {
    let mut row_pos = 0;

    for color in image {
        print!("{}", if *color == WHITE {'#'} else {' ' });

        row_pos += 1;
        if row_pos == COLS {
            row_pos = 0;
            println!();
        }
    }
}
