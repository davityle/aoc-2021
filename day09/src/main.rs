use std::collections::HashSet;
use std::num::Wrapping;

fn main() {
    let height_map = INPUT;

    let mut categorized: HashSet<(usize, usize)> = HashSet::new();
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();

    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            if categorized.contains(&(x, y)) || height_map[y][x] == 9 {
                continue;
            }
            let mut basin: HashSet<(usize, usize)> = HashSet::new();
            create_basin(x, y, height_map, &mut basin);
            for point in basin.iter() {
                categorized.insert((point.0, point.1));
            }
            basins.push(basin);
        }
    }

    basins.sort_by_key(|b| b.len());

    println!(
        "{}",
        basins
            .iter()
            .rev()
            .take(3)
            .map(|b| b.len())
            .fold(1, |a, b| a * b)
    );
}

fn create_basin(x: usize, y: usize, height_map: InputType, basin: &mut HashSet<(usize, usize)>) {
    if x >= 100 || y >= 100 || height_map[y][x] == 9 || basin.contains(&(x, y)) {
        return;
    }
    basin.insert((x, y));
    create_basin(x, subtract_one(y), height_map, basin);
    create_basin(x, y + 1, height_map, basin);
    create_basin(subtract_one(x), y, height_map, basin);
    create_basin(x + 1, y, height_map, basin);
}

fn subtract_one(n: usize) -> usize {
    (Wrapping(n) - Wrapping(1)).0
}

type InputType = [[u8; 100]; 100];

const _: [[u8; 10]; 5] = [
    [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
    [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
    [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
    [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
    [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
];

const INPUT: [[u8; 100]; 100] = [
    [
        7, 6, 7, 8, 9, 2, 1, 2, 3, 4, 9, 8, 8, 6, 7, 8, 9, 0, 1, 2, 3, 8, 9, 5, 4, 3, 2, 3, 4, 9,
        8, 7, 6, 5, 4, 3, 2, 1, 2, 5, 7, 8, 9, 9, 9, 9, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 5, 4, 2, 2,
        3, 7, 8, 9, 9, 8, 9, 8, 6, 5, 4, 3, 2, 1, 2, 4, 5, 6, 9, 8, 9, 9, 8, 9, 4, 3, 2, 5, 6, 7,
        8, 9, 8, 9, 2, 1, 2, 9, 6, 5,
    ],
    [
        6, 5, 6, 7, 8, 9, 0, 1, 3, 9, 8, 7, 6, 5, 8, 9, 9, 9, 2, 3, 6, 7, 8, 9, 6, 5, 1, 2, 9, 8,
        7, 6, 5, 4, 3, 2, 1, 0, 3, 4, 5, 6, 9, 9, 8, 8, 9, 7, 9, 9, 9, 9, 8, 7, 9, 3, 2, 1, 0, 1,
        2, 5, 6, 9, 8, 7, 6, 9, 8, 7, 5, 7, 3, 2, 4, 5, 9, 7, 9, 7, 6, 7, 7, 9, 9, 4, 3, 4, 5, 9,
        9, 8, 7, 8, 9, 0, 9, 8, 9, 3,
    ],
    [
        5, 4, 5, 6, 7, 9, 9, 2, 9, 8, 9, 8, 7, 6, 9, 9, 9, 8, 9, 4, 5, 6, 7, 8, 9, 4, 0, 1, 3, 9,
        8, 7, 6, 5, 7, 3, 2, 1, 2, 5, 6, 7, 8, 9, 6, 7, 9, 9, 1, 0, 9, 8, 9, 8, 8, 6, 3, 2, 1, 2,
        3, 4, 5, 8, 9, 6, 5, 9, 8, 7, 6, 8, 4, 3, 4, 7, 8, 9, 8, 9, 5, 5, 6, 8, 8, 9, 7, 6, 9, 8,
        8, 7, 6, 8, 9, 1, 9, 7, 8, 9,
    ],
    [
        4, 3, 4, 5, 8, 9, 8, 9, 8, 7, 6, 9, 8, 7, 8, 9, 8, 7, 8, 9, 6, 7, 8, 9, 4, 3, 2, 3, 4, 9,
        9, 9, 7, 7, 5, 4, 3, 2, 4, 5, 7, 8, 9, 4, 5, 6, 7, 8, 9, 9, 8, 7, 9, 9, 9, 5, 4, 4, 3, 4,
        5, 5, 6, 7, 8, 9, 4, 3, 9, 8, 7, 8, 9, 4, 5, 8, 9, 9, 7, 5, 4, 4, 5, 6, 7, 8, 9, 7, 9, 7,
        6, 9, 5, 6, 8, 9, 8, 6, 7, 8,
    ],
    [
        3, 2, 1, 2, 9, 8, 7, 6, 5, 6, 5, 6, 9, 8, 9, 8, 7, 6, 7, 9, 9, 8, 9, 8, 5, 4, 3, 5, 9, 8,
        9, 8, 9, 9, 7, 6, 4, 7, 6, 6, 7, 8, 9, 3, 6, 6, 7, 8, 9, 8, 7, 5, 9, 8, 7, 6, 5, 6, 7, 7,
        6, 6, 7, 8, 9, 9, 3, 2, 3, 9, 8, 9, 6, 5, 6, 9, 9, 8, 7, 6, 3, 2, 4, 5, 6, 7, 8, 9, 8, 6,
        5, 4, 3, 2, 2, 3, 4, 5, 6, 7,
    ],
    [
        4, 3, 3, 6, 7, 9, 6, 5, 4, 3, 4, 5, 6, 9, 8, 9, 6, 5, 8, 7, 8, 9, 9, 7, 6, 5, 4, 9, 9, 7,
        8, 7, 9, 8, 7, 6, 5, 6, 7, 7, 8, 9, 0, 1, 5, 5, 8, 9, 8, 9, 6, 4, 2, 9, 8, 7, 6, 7, 8, 9,
        7, 8, 9, 9, 7, 8, 9, 1, 2, 3, 9, 8, 7, 6, 7, 8, 9, 9, 8, 8, 7, 4, 5, 6, 7, 8, 9, 3, 9, 8,
        4, 3, 2, 1, 0, 4, 5, 6, 9, 8,
    ],
    [
        5, 4, 4, 5, 9, 9, 5, 4, 3, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5, 6, 6, 9, 9, 8, 7, 6, 9, 8, 7, 6,
        6, 6, 8, 9, 9, 7, 6, 7, 9, 8, 9, 3, 1, 2, 3, 4, 9, 8, 7, 6, 5, 4, 1, 0, 9, 9, 8, 8, 9, 9,
        9, 9, 3, 5, 6, 7, 8, 9, 3, 9, 8, 9, 9, 7, 8, 9, 6, 7, 9, 9, 6, 5, 6, 7, 8, 9, 1, 2, 9, 9,
        5, 4, 5, 2, 1, 6, 6, 7, 8, 9,
    ],
    [
        6, 5, 6, 9, 8, 7, 6, 5, 2, 1, 2, 3, 6, 7, 8, 9, 1, 2, 3, 4, 5, 8, 9, 9, 8, 9, 8, 7, 6, 5,
        4, 5, 6, 7, 8, 9, 9, 8, 9, 9, 5, 4, 5, 3, 5, 6, 9, 9, 8, 5, 4, 3, 2, 1, 2, 3, 9, 9, 9, 9,
        0, 1, 2, 6, 9, 8, 9, 2, 9, 8, 7, 5, 9, 8, 9, 4, 5, 7, 9, 8, 7, 8, 7, 8, 9, 4, 2, 9, 8, 7,
        6, 7, 4, 3, 2, 7, 8, 9, 9, 5,
    ],
    [
        7, 6, 8, 9, 9, 8, 6, 4, 3, 2, 3, 4, 5, 6, 9, 1, 0, 1, 2, 5, 6, 7, 9, 9, 9, 0, 9, 8, 7, 6,
        7, 6, 7, 8, 9, 7, 6, 9, 8, 9, 8, 7, 6, 8, 7, 7, 8, 9, 9, 9, 8, 4, 3, 2, 3, 4, 5, 6, 7, 8,
        9, 2, 3, 7, 8, 9, 2, 1, 2, 9, 6, 4, 2, 9, 2, 3, 4, 9, 9, 9, 8, 9, 8, 9, 6, 5, 4, 5, 9, 8,
        7, 8, 9, 5, 3, 4, 5, 9, 5, 4,
    ],
    [
        8, 7, 8, 9, 9, 7, 6, 5, 4, 5, 8, 5, 6, 7, 8, 9, 1, 3, 4, 5, 6, 9, 8, 9, 2, 1, 9, 9, 8, 9,
        8, 7, 8, 9, 8, 9, 5, 9, 7, 6, 9, 8, 9, 9, 8, 8, 9, 9, 9, 8, 7, 6, 5, 6, 7, 9, 6, 7, 8, 9,
        8, 3, 5, 6, 7, 9, 9, 2, 9, 8, 7, 3, 1, 0, 1, 9, 9, 8, 9, 9, 9, 7, 9, 9, 8, 6, 5, 6, 7, 9,
        8, 9, 8, 5, 4, 5, 7, 8, 9, 3,
    ],
    [
        9, 8, 9, 5, 9, 8, 9, 6, 8, 6, 7, 8, 7, 8, 9, 9, 2, 4, 6, 6, 9, 8, 7, 8, 9, 9, 8, 9, 9, 9,
        9, 8, 9, 8, 7, 6, 9, 8, 9, 4, 3, 9, 4, 3, 9, 9, 9, 8, 9, 9, 8, 7, 6, 7, 8, 9, 8, 8, 9, 8,
        7, 5, 7, 7, 8, 9, 8, 9, 8, 7, 6, 5, 6, 1, 9, 8, 9, 6, 8, 9, 5, 6, 9, 9, 9, 8, 9, 7, 9, 9,
        9, 8, 7, 6, 5, 6, 8, 9, 5, 4,
    ],
    [
        4, 9, 3, 4, 3, 9, 9, 7, 8, 7, 8, 9, 9, 9, 9, 7, 6, 5, 7, 9, 9, 7, 6, 7, 9, 8, 7, 9, 9, 8,
        7, 9, 5, 7, 6, 5, 6, 7, 9, 3, 2, 3, 9, 4, 9, 8, 7, 6, 5, 6, 9, 9, 7, 8, 9, 9, 9, 9, 5, 9,
        9, 9, 8, 8, 9, 6, 7, 7, 9, 8, 8, 6, 8, 9, 8, 7, 6, 5, 7, 8, 4, 9, 8, 9, 9, 9, 8, 9, 8, 7,
        9, 9, 8, 7, 6, 7, 9, 7, 6, 5,
    ],
    [
        3, 2, 1, 0, 2, 3, 9, 8, 9, 8, 9, 8, 9, 2, 9, 8, 7, 9, 9, 8, 7, 6, 5, 3, 4, 5, 6, 8, 8, 9,
        6, 5, 4, 3, 2, 3, 4, 8, 9, 9, 3, 9, 8, 9, 9, 7, 6, 5, 4, 2, 4, 9, 8, 9, 9, 9, 2, 3, 4, 7,
        8, 9, 9, 9, 4, 5, 6, 6, 7, 9, 8, 7, 9, 8, 7, 6, 5, 4, 3, 4, 3, 4, 6, 7, 8, 9, 7, 6, 7, 5,
        6, 7, 9, 8, 7, 9, 9, 8, 7, 6,
    ],
    [
        4, 3, 2, 3, 3, 5, 9, 9, 9, 9, 8, 7, 9, 1, 2, 9, 9, 8, 7, 6, 5, 5, 4, 2, 3, 4, 5, 6, 7, 8,
        9, 6, 5, 4, 4, 5, 5, 6, 7, 8, 9, 6, 6, 7, 8, 9, 7, 4, 3, 1, 3, 4, 9, 9, 8, 8, 9, 9, 5, 6,
        7, 9, 6, 4, 3, 4, 5, 5, 6, 9, 9, 9, 8, 7, 6, 5, 4, 3, 2, 3, 2, 3, 4, 5, 9, 7, 6, 5, 4, 3,
        3, 4, 5, 9, 8, 9, 3, 9, 8, 7,
    ],
    [
        5, 5, 3, 5, 6, 9, 8, 9, 9, 8, 9, 6, 8, 9, 3, 4, 5, 9, 9, 5, 4, 3, 0, 1, 2, 3, 4, 8, 9, 9,
        8, 7, 6, 6, 5, 6, 6, 7, 8, 9, 3, 5, 5, 6, 7, 8, 9, 5, 1, 0, 3, 4, 9, 9, 7, 6, 7, 8, 9, 7,
        8, 9, 5, 4, 2, 3, 3, 4, 9, 8, 9, 9, 9, 9, 8, 8, 7, 5, 1, 0, 1, 2, 5, 7, 8, 9, 7, 8, 7, 1,
        2, 7, 7, 8, 9, 3, 2, 3, 9, 8,
    ],
    [
        6, 6, 4, 5, 9, 8, 7, 8, 9, 7, 6, 5, 9, 5, 4, 9, 6, 9, 8, 7, 9, 2, 1, 3, 3, 9, 5, 9, 4, 2,
        9, 9, 8, 7, 6, 7, 9, 8, 9, 1, 2, 3, 4, 5, 8, 9, 4, 3, 2, 1, 2, 9, 8, 6, 5, 5, 6, 7, 8, 9,
        9, 5, 4, 3, 1, 0, 2, 9, 8, 7, 8, 9, 9, 8, 7, 6, 5, 4, 2, 1, 2, 3, 4, 5, 7, 8, 9, 7, 6, 5,
        3, 5, 6, 7, 8, 9, 0, 1, 2, 9,
    ],
    [
        9, 7, 5, 9, 8, 7, 6, 7, 8, 9, 7, 9, 8, 9, 9, 8, 9, 9, 9, 9, 8, 3, 2, 4, 9, 8, 9, 8, 9, 0,
        2, 3, 9, 8, 7, 8, 9, 9, 3, 2, 3, 5, 6, 7, 9, 6, 5, 4, 3, 2, 9, 8, 7, 5, 4, 3, 4, 6, 7, 8,
        9, 6, 4, 3, 2, 2, 9, 8, 7, 6, 5, 5, 3, 9, 8, 7, 8, 5, 4, 3, 4, 8, 5, 7, 8, 9, 9, 8, 7, 6,
        5, 7, 7, 8, 9, 2, 1, 2, 3, 4,
    ],
    [
        9, 8, 9, 8, 7, 6, 5, 9, 9, 0, 9, 8, 7, 8, 7, 7, 6, 9, 8, 7, 6, 5, 3, 9, 8, 7, 6, 7, 8, 9,
        3, 5, 6, 9, 8, 9, 8, 7, 5, 6, 7, 8, 9, 8, 9, 7, 9, 5, 4, 3, 4, 9, 8, 5, 3, 2, 4, 5, 6, 7,
        8, 9, 9, 4, 3, 4, 6, 9, 8, 5, 4, 3, 2, 3, 9, 8, 7, 6, 5, 4, 6, 7, 6, 8, 9, 6, 8, 9, 9, 7,
        8, 9, 8, 9, 4, 3, 4, 3, 7, 8,
    ],
    [
        9, 9, 4, 9, 9, 7, 6, 8, 9, 1, 3, 9, 6, 6, 6, 2, 5, 4, 9, 8, 7, 6, 9, 8, 9, 7, 5, 6, 7, 8,
        9, 6, 9, 8, 9, 9, 8, 7, 6, 7, 8, 9, 3, 9, 5, 9, 8, 7, 6, 4, 9, 8, 4, 3, 2, 1, 3, 4, 6, 7,
        9, 9, 8, 9, 4, 5, 9, 8, 7, 6, 6, 5, 4, 4, 5, 9, 8, 7, 6, 5, 6, 7, 8, 9, 6, 5, 6, 7, 9, 8,
        9, 9, 9, 7, 5, 4, 5, 4, 5, 6,
    ],
    [
        9, 9, 5, 6, 9, 8, 7, 9, 3, 2, 9, 8, 5, 4, 5, 1, 2, 3, 4, 9, 8, 9, 9, 7, 7, 6, 4, 5, 6, 5,
        6, 9, 9, 7, 6, 4, 9, 9, 9, 8, 9, 3, 2, 3, 4, 5, 9, 8, 7, 9, 8, 7, 5, 4, 3, 2, 4, 5, 7, 8,
        9, 8, 7, 9, 5, 6, 9, 9, 9, 8, 7, 8, 6, 8, 6, 7, 9, 9, 7, 6, 7, 8, 9, 8, 7, 4, 9, 9, 7, 9,
        7, 8, 9, 7, 6, 9, 7, 9, 6, 7,
    ],
    [
        7, 8, 9, 9, 9, 9, 9, 5, 4, 9, 8, 7, 6, 3, 1, 0, 1, 2, 5, 6, 9, 8, 7, 6, 5, 4, 3, 2, 3, 4,
        5, 8, 9, 6, 4, 3, 2, 1, 0, 9, 9, 8, 5, 4, 5, 6, 7, 9, 8, 9, 9, 8, 6, 7, 6, 3, 4, 6, 7, 8,
        9, 5, 6, 7, 9, 9, 8, 7, 5, 9, 8, 9, 7, 8, 7, 8, 9, 9, 8, 7, 8, 9, 5, 9, 9, 9, 8, 6, 6, 5,
        6, 7, 8, 9, 9, 8, 9, 8, 9, 8,
    ],
    [
        6, 7, 9, 8, 8, 9, 7, 6, 5, 6, 9, 9, 5, 4, 2, 1, 2, 3, 4, 9, 9, 9, 8, 7, 6, 5, 0, 1, 4, 5,
        6, 7, 8, 9, 5, 4, 3, 9, 9, 8, 9, 7, 6, 5, 6, 7, 8, 9, 9, 9, 8, 9, 7, 8, 5, 4, 5, 6, 8, 9,
        4, 4, 5, 6, 7, 8, 9, 5, 4, 3, 9, 9, 8, 9, 8, 9, 5, 5, 9, 8, 9, 5, 4, 9, 8, 7, 6, 4, 3, 4,
        5, 8, 9, 9, 8, 7, 8, 7, 8, 9,
    ],
    [
        5, 6, 7, 6, 7, 8, 9, 7, 9, 9, 8, 7, 6, 5, 3, 2, 4, 5, 9, 8, 9, 9, 9, 9, 7, 6, 3, 2, 5, 6,
        7, 8, 9, 7, 6, 6, 9, 8, 8, 7, 9, 8, 7, 6, 7, 8, 9, 8, 9, 9, 7, 6, 9, 9, 8, 7, 6, 7, 9, 4,
        3, 3, 4, 5, 6, 7, 8, 9, 3, 2, 1, 0, 9, 9, 9, 5, 4, 4, 6, 9, 9, 6, 9, 8, 7, 6, 5, 3, 2, 3,
        4, 9, 9, 8, 7, 6, 5, 6, 7, 8,
    ],
    [
        4, 5, 6, 5, 6, 9, 2, 9, 8, 7, 9, 8, 7, 6, 8, 3, 7, 6, 9, 7, 8, 9, 8, 7, 6, 5, 4, 3, 4, 7,
        8, 9, 9, 8, 7, 9, 8, 7, 5, 6, 7, 9, 9, 7, 9, 9, 8, 7, 8, 9, 9, 5, 4, 3, 9, 8, 8, 9, 3, 2,
        1, 2, 3, 4, 5, 8, 9, 5, 4, 3, 3, 1, 9, 8, 6, 5, 3, 2, 5, 7, 8, 9, 8, 7, 6, 5, 4, 3, 1, 2,
        9, 9, 8, 7, 6, 4, 4, 5, 6, 9,
    ],
    [
        3, 2, 3, 4, 8, 9, 1, 9, 9, 6, 5, 9, 8, 9, 5, 4, 5, 9, 8, 6, 9, 8, 9, 9, 8, 6, 7, 4, 9, 8,
        9, 4, 3, 9, 9, 7, 9, 5, 4, 5, 6, 7, 8, 9, 9, 8, 9, 5, 6, 9, 8, 9, 4, 2, 1, 9, 9, 5, 4, 9,
        5, 3, 4, 5, 6, 7, 8, 9, 5, 4, 4, 2, 9, 8, 7, 6, 4, 3, 4, 5, 6, 7, 9, 9, 8, 6, 5, 2, 0, 9,
        8, 9, 9, 9, 4, 3, 2, 8, 7, 8,
    ],
    [
        4, 5, 4, 5, 6, 8, 9, 8, 7, 9, 4, 5, 9, 7, 6, 5, 9, 8, 6, 5, 6, 7, 8, 9, 9, 8, 7, 6, 8, 9,
        6, 5, 2, 9, 8, 6, 5, 4, 3, 6, 5, 6, 7, 9, 8, 7, 5, 4, 5, 6, 7, 8, 9, 3, 9, 8, 7, 6, 9, 8,
        9, 4, 5, 9, 7, 8, 9, 8, 6, 7, 8, 3, 4, 9, 8, 7, 5, 4, 5, 6, 9, 9, 8, 9, 9, 7, 5, 4, 9, 8,
        7, 8, 9, 8, 5, 4, 1, 2, 4, 5,
    ],
    [
        7, 6, 6, 6, 7, 9, 9, 7, 6, 7, 2, 4, 9, 8, 9, 9, 8, 6, 5, 4, 5, 6, 7, 7, 8, 9, 8, 7, 8, 9,
        8, 6, 9, 8, 9, 6, 5, 3, 2, 3, 4, 5, 7, 9, 7, 6, 4, 3, 4, 5, 6, 7, 8, 9, 1, 9, 8, 9, 8, 7,
        9, 5, 6, 7, 8, 9, 9, 9, 8, 9, 6, 5, 5, 6, 9, 8, 9, 5, 6, 9, 8, 9, 7, 9, 8, 9, 9, 9, 8, 7,
        6, 9, 8, 7, 6, 7, 2, 3, 9, 6,
    ],
    [
        8, 7, 8, 7, 8, 9, 8, 6, 5, 4, 1, 2, 3, 9, 9, 9, 7, 9, 4, 3, 2, 5, 7, 6, 7, 9, 9, 8, 9, 9,
        9, 9, 8, 7, 9, 7, 9, 4, 1, 2, 3, 9, 9, 8, 5, 4, 3, 2, 3, 4, 5, 7, 8, 9, 2, 4, 9, 9, 9, 6,
        8, 9, 9, 8, 9, 9, 8, 9, 9, 8, 7, 7, 6, 7, 8, 9, 7, 6, 9, 8, 7, 6, 5, 8, 7, 9, 8, 9, 7, 6,
        5, 6, 9, 8, 9, 8, 3, 9, 8, 9,
    ],
    [
        9, 8, 9, 8, 9, 8, 7, 6, 5, 3, 2, 3, 4, 5, 9, 8, 6, 4, 3, 2, 1, 4, 6, 5, 6, 8, 9, 9, 9, 9,
        8, 8, 7, 6, 8, 9, 8, 9, 2, 4, 9, 8, 7, 6, 4, 3, 2, 1, 2, 3, 5, 6, 7, 8, 9, 5, 9, 8, 9, 5,
        6, 9, 8, 9, 7, 6, 7, 8, 9, 9, 9, 8, 9, 8, 9, 9, 8, 8, 9, 9, 7, 6, 4, 6, 5, 6, 7, 8, 9, 2,
        4, 5, 8, 9, 9, 9, 9, 8, 7, 8,
    ],
    [
        8, 9, 5, 9, 1, 9, 9, 7, 6, 4, 5, 4, 5, 6, 8, 9, 9, 5, 4, 9, 0, 2, 3, 4, 5, 7, 8, 9, 9, 8,
        7, 7, 6, 5, 7, 8, 6, 8, 9, 9, 8, 7, 6, 5, 4, 2, 1, 0, 1, 2, 4, 5, 6, 7, 8, 9, 8, 7, 5, 4,
        7, 6, 7, 8, 9, 5, 8, 8, 9, 9, 8, 9, 8, 9, 6, 7, 9, 9, 9, 9, 8, 4, 3, 2, 4, 5, 6, 9, 0, 1,
        2, 3, 6, 7, 8, 9, 9, 7, 6, 7,
    ],
    [
        7, 5, 4, 3, 0, 1, 9, 8, 7, 8, 7, 8, 9, 8, 9, 9, 8, 9, 9, 8, 9, 5, 4, 9, 6, 8, 9, 9, 8, 7,
        6, 6, 4, 3, 2, 4, 5, 6, 8, 9, 9, 8, 7, 6, 5, 3, 2, 1, 2, 3, 5, 7, 8, 9, 9, 4, 9, 8, 6, 3,
        6, 5, 6, 9, 4, 4, 6, 7, 9, 8, 7, 6, 7, 8, 5, 6, 7, 8, 9, 9, 9, 9, 3, 1, 3, 4, 5, 8, 9, 2,
        3, 4, 5, 7, 9, 9, 8, 7, 5, 8,
    ],
    [
        9, 6, 5, 2, 1, 2, 3, 9, 8, 9, 8, 9, 6, 9, 9, 8, 7, 8, 9, 7, 9, 9, 9, 8, 9, 9, 0, 1, 9, 8,
        5, 4, 3, 2, 1, 3, 6, 7, 9, 3, 2, 9, 8, 7, 6, 5, 3, 2, 3, 7, 6, 8, 9, 1, 2, 3, 9, 9, 9, 2,
        3, 4, 7, 9, 2, 3, 4, 9, 8, 7, 6, 5, 4, 5, 4, 5, 9, 9, 8, 8, 9, 8, 9, 2, 6, 5, 6, 7, 8, 9,
        4, 5, 6, 8, 9, 8, 9, 5, 4, 9,
    ],
    [
        8, 7, 8, 3, 4, 3, 5, 6, 9, 9, 9, 3, 4, 9, 8, 7, 6, 5, 4, 6, 7, 8, 9, 7, 8, 9, 1, 9, 8, 7,
        6, 5, 4, 3, 0, 4, 6, 7, 8, 9, 1, 0, 9, 8, 9, 5, 4, 3, 4, 8, 7, 9, 1, 0, 1, 9, 8, 9, 8, 1,
        2, 5, 6, 8, 9, 9, 5, 6, 9, 8, 5, 4, 3, 2, 3, 9, 8, 9, 6, 7, 8, 7, 9, 8, 7, 6, 7, 8, 9, 8,
        5, 8, 7, 9, 8, 7, 6, 4, 3, 5,
    ],
    [
        9, 8, 9, 7, 5, 4, 5, 7, 8, 9, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 3, 4, 5, 6, 7, 9, 2, 3, 9, 8,
        7, 6, 5, 2, 1, 6, 7, 8, 9, 8, 9, 1, 2, 9, 7, 6, 5, 4, 5, 6, 8, 9, 2, 3, 9, 8, 7, 8, 6, 0,
        1, 4, 7, 7, 8, 8, 9, 7, 8, 9, 6, 3, 2, 1, 9, 8, 7, 8, 5, 6, 7, 6, 7, 9, 8, 7, 9, 9, 9, 7,
        6, 9, 8, 9, 9, 8, 9, 3, 2, 4,
    ],
    [
        9, 9, 9, 8, 6, 7, 6, 7, 9, 2, 1, 0, 1, 9, 9, 7, 6, 8, 9, 3, 4, 5, 6, 9, 8, 9, 3, 4, 5, 9,
        8, 7, 4, 3, 9, 9, 9, 9, 8, 7, 8, 9, 3, 9, 8, 7, 8, 6, 6, 7, 8, 9, 3, 9, 8, 7, 6, 5, 4, 1,
        2, 3, 4, 5, 6, 7, 8, 9, 9, 8, 7, 4, 4, 9, 8, 7, 6, 7, 4, 3, 4, 5, 8, 8, 9, 8, 9, 9, 8, 9,
        9, 9, 9, 2, 3, 9, 8, 4, 3, 5,
    ],
    [
        8, 7, 9, 9, 7, 8, 7, 8, 9, 3, 9, 1, 2, 3, 9, 8, 7, 9, 5, 4, 5, 8, 7, 8, 9, 5, 4, 5, 6, 8,
        9, 8, 9, 9, 8, 8, 7, 8, 7, 6, 6, 8, 9, 7, 9, 8, 9, 7, 8, 9, 9, 6, 9, 8, 9, 8, 7, 6, 3, 2,
        3, 4, 5, 6, 8, 8, 9, 8, 9, 8, 7, 5, 9, 8, 7, 6, 5, 4, 3, 2, 3, 4, 5, 7, 8, 9, 9, 9, 7, 8,
        8, 9, 2, 1, 9, 8, 7, 5, 6, 7,
    ],
    [
        7, 6, 7, 8, 9, 9, 8, 9, 5, 9, 8, 9, 3, 4, 5, 9, 9, 8, 7, 5, 6, 7, 8, 9, 9, 7, 6, 6, 7, 9,
        8, 9, 9, 8, 7, 7, 6, 5, 6, 4, 5, 9, 9, 6, 4, 9, 9, 8, 9, 5, 6, 5, 8, 7, 8, 9, 8, 9, 5, 5,
        4, 5, 6, 7, 9, 9, 6, 7, 8, 9, 7, 6, 9, 9, 8, 5, 4, 3, 2, 1, 2, 3, 6, 9, 9, 9, 9, 8, 6, 7,
        7, 8, 9, 2, 3, 9, 8, 8, 7, 8,
    ],
    [
        6, 5, 7, 8, 9, 9, 9, 4, 4, 6, 7, 8, 9, 5, 6, 7, 8, 9, 9, 6, 7, 8, 9, 9, 8, 9, 7, 7, 8, 9,
        7, 8, 9, 9, 6, 5, 6, 4, 5, 3, 8, 7, 8, 9, 3, 2, 1, 9, 9, 4, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6,
        5, 6, 7, 8, 9, 4, 5, 6, 9, 8, 9, 7, 8, 9, 9, 6, 5, 4, 3, 4, 3, 4, 5, 8, 9, 9, 8, 7, 5, 4,
        5, 7, 9, 9, 4, 6, 9, 9, 8, 9,
    ],
    [
        3, 4, 6, 7, 8, 9, 1, 2, 3, 5, 6, 9, 7, 6, 7, 9, 9, 9, 8, 7, 9, 9, 5, 6, 7, 8, 9, 8, 9, 8,
        6, 7, 9, 8, 7, 4, 3, 2, 1, 2, 3, 6, 7, 9, 4, 3, 0, 9, 8, 9, 2, 3, 4, 5, 6, 9, 8, 9, 9, 7,
        6, 7, 8, 9, 4, 3, 4, 9, 8, 7, 8, 9, 9, 9, 8, 7, 6, 5, 5, 5, 4, 6, 6, 7, 8, 9, 9, 8, 6, 2,
        4, 6, 7, 8, 9, 7, 8, 9, 9, 8,
    ],
    [
        2, 3, 7, 9, 9, 1, 0, 1, 4, 5, 7, 8, 9, 7, 9, 8, 9, 6, 9, 8, 9, 6, 4, 6, 6, 7, 8, 9, 8, 7,
        5, 6, 6, 9, 8, 5, 3, 2, 0, 1, 4, 5, 8, 9, 5, 4, 9, 8, 7, 8, 9, 7, 5, 9, 9, 8, 7, 8, 9, 9,
        9, 8, 9, 8, 5, 5, 9, 8, 7, 6, 7, 8, 9, 8, 9, 9, 8, 9, 7, 6, 7, 7, 8, 8, 9, 3, 5, 9, 5, 3,
        4, 5, 6, 7, 8, 9, 9, 9, 8, 7,
    ],
    [
        9, 4, 9, 8, 9, 2, 1, 2, 3, 4, 5, 7, 8, 9, 6, 7, 3, 4, 5, 9, 1, 2, 3, 4, 5, 6, 9, 8, 6, 5,
        4, 3, 5, 6, 9, 6, 5, 3, 4, 2, 3, 6, 7, 8, 9, 9, 8, 7, 6, 7, 9, 8, 9, 8, 9, 7, 6, 7, 8, 9,
        9, 9, 8, 7, 6, 9, 8, 8, 6, 5, 6, 7, 8, 7, 8, 9, 9, 9, 8, 7, 8, 8, 9, 9, 0, 2, 9, 7, 6, 4,
        5, 6, 7, 8, 9, 9, 9, 8, 9, 6,
    ],
    [
        8, 9, 9, 7, 9, 4, 5, 9, 4, 5, 6, 8, 9, 6, 5, 3, 2, 3, 9, 5, 4, 3, 4, 5, 6, 7, 8, 9, 7, 4,
        3, 2, 3, 9, 8, 7, 7, 4, 6, 3, 4, 7, 8, 9, 9, 8, 9, 8, 5, 6, 7, 9, 9, 7, 5, 6, 5, 6, 7, 8,
        9, 9, 9, 8, 9, 8, 7, 6, 5, 4, 2, 3, 5, 6, 9, 9, 8, 6, 9, 8, 9, 9, 4, 5, 9, 9, 8, 7, 6, 5,
        7, 7, 8, 9, 9, 9, 8, 7, 6, 5,
    ],
    [
        7, 6, 8, 6, 8, 9, 9, 8, 9, 9, 7, 8, 9, 7, 6, 4, 3, 9, 8, 6, 5, 4, 6, 7, 8, 9, 9, 9, 8, 9,
        9, 3, 4, 5, 9, 9, 8, 5, 6, 9, 9, 8, 9, 7, 6, 7, 5, 6, 4, 5, 7, 8, 9, 5, 4, 5, 4, 5, 6, 8,
        9, 9, 8, 9, 5, 9, 9, 8, 7, 5, 1, 2, 3, 9, 8, 7, 6, 5, 4, 9, 1, 2, 3, 4, 8, 9, 9, 9, 7, 6,
        7, 8, 9, 9, 9, 7, 9, 6, 5, 4,
    ],
    [
        2, 5, 4, 5, 7, 9, 8, 7, 8, 8, 9, 9, 9, 8, 7, 5, 4, 9, 8, 7, 9, 8, 7, 8, 9, 9, 8, 7, 9, 8,
        8, 9, 5, 7, 9, 8, 7, 6, 7, 8, 9, 9, 7, 6, 5, 4, 3, 5, 3, 9, 8, 9, 5, 4, 3, 2, 3, 4, 5, 6,
        9, 8, 7, 6, 4, 2, 1, 9, 8, 6, 0, 1, 3, 4, 9, 8, 7, 6, 3, 2, 0, 1, 5, 6, 7, 8, 9, 9, 8, 7,
        8, 9, 9, 8, 7, 6, 5, 4, 2, 3,
    ],
    [
        1, 2, 3, 6, 7, 8, 9, 6, 5, 7, 8, 9, 9, 9, 8, 9, 5, 6, 9, 8, 9, 9, 8, 9, 9, 9, 8, 6, 7, 6,
        7, 9, 9, 8, 9, 9, 8, 9, 8, 9, 9, 8, 7, 5, 4, 3, 2, 1, 2, 8, 9, 5, 4, 3, 2, 1, 2, 3, 6, 9,
        8, 7, 6, 5, 3, 1, 0, 1, 9, 7, 1, 2, 4, 5, 6, 9, 6, 5, 4, 3, 2, 3, 4, 6, 8, 9, 9, 8, 9, 9,
        9, 7, 6, 9, 8, 7, 6, 5, 3, 4,
    ],
    [
        0, 1, 9, 8, 9, 9, 6, 5, 4, 5, 6, 7, 8, 9, 9, 7, 6, 8, 9, 9, 6, 8, 9, 0, 9, 8, 6, 5, 4, 5,
        6, 7, 8, 9, 9, 8, 9, 7, 9, 9, 8, 9, 8, 9, 8, 7, 3, 2, 3, 7, 8, 9, 5, 4, 3, 2, 5, 4, 7, 8,
        9, 8, 9, 8, 7, 2, 5, 2, 3, 9, 2, 9, 5, 9, 9, 8, 7, 6, 7, 4, 3, 4, 5, 6, 9, 9, 8, 7, 9, 8,
        7, 6, 5, 6, 9, 9, 8, 6, 7, 6,
    ],
    [
        1, 2, 4, 6, 7, 8, 9, 3, 2, 4, 5, 6, 8, 9, 9, 8, 7, 9, 8, 7, 5, 4, 2, 1, 9, 8, 7, 4, 3, 6,
        7, 8, 9, 9, 8, 7, 5, 6, 9, 8, 7, 6, 9, 9, 9, 6, 5, 3, 4, 6, 7, 9, 6, 5, 4, 3, 4, 5, 6, 7,
        9, 9, 9, 7, 6, 5, 4, 3, 5, 6, 9, 8, 9, 8, 9, 9, 8, 9, 6, 5, 4, 5, 6, 7, 8, 9, 7, 6, 8, 9,
        8, 5, 4, 3, 4, 5, 9, 9, 8, 7,
    ],
    [
        2, 3, 4, 5, 9, 9, 4, 2, 1, 3, 5, 7, 9, 3, 2, 9, 9, 9, 9, 8, 6, 4, 3, 4, 6, 9, 7, 6, 7, 8,
        8, 9, 8, 7, 6, 5, 4, 3, 9, 8, 6, 5, 7, 9, 8, 7, 7, 8, 5, 7, 8, 9, 8, 7, 6, 4, 5, 6, 7, 8,
        9, 9, 9, 9, 7, 6, 5, 4, 5, 9, 8, 7, 8, 7, 8, 8, 9, 8, 7, 6, 9, 8, 7, 8, 9, 8, 7, 5, 7, 9,
        5, 4, 3, 2, 3, 4, 6, 8, 9, 9,
    ],
    [
        3, 4, 5, 6, 9, 4, 3, 1, 0, 3, 5, 8, 9, 4, 3, 9, 8, 9, 9, 9, 7, 6, 4, 8, 7, 9, 8, 7, 8, 9,
        9, 3, 9, 8, 9, 6, 6, 4, 9, 7, 5, 4, 6, 7, 9, 8, 9, 9, 9, 8, 9, 7, 9, 8, 9, 5, 9, 7, 8, 9,
        9, 8, 9, 9, 9, 7, 6, 5, 9, 8, 7, 6, 5, 6, 9, 7, 8, 9, 8, 7, 8, 9, 8, 9, 5, 9, 3, 4, 9, 8,
        9, 5, 2, 1, 2, 3, 5, 6, 7, 8,
    ],
    [
        6, 5, 7, 7, 8, 9, 3, 2, 1, 2, 3, 4, 7, 9, 9, 8, 7, 9, 9, 9, 8, 7, 8, 9, 8, 9, 9, 8, 9, 8,
        9, 2, 0, 9, 8, 9, 7, 9, 8, 9, 4, 3, 4, 9, 8, 9, 3, 2, 1, 9, 5, 6, 8, 9, 9, 9, 8, 9, 9, 9,
        9, 7, 8, 9, 9, 8, 7, 6, 9, 9, 6, 5, 4, 3, 4, 6, 8, 8, 9, 8, 9, 7, 9, 5, 4, 3, 2, 9, 8, 7,
        8, 9, 4, 5, 6, 5, 6, 7, 8, 9,
    ],
    [
        7, 8, 9, 8, 9, 5, 4, 5, 2, 3, 4, 5, 6, 8, 9, 5, 6, 8, 8, 9, 9, 9, 9, 9, 9, 8, 7, 9, 8, 7,
        8, 9, 9, 8, 7, 8, 9, 8, 7, 9, 2, 1, 9, 8, 7, 6, 5, 3, 2, 3, 4, 8, 9, 9, 8, 9, 7, 8, 9, 8,
        7, 6, 7, 8, 9, 9, 9, 9, 8, 7, 5, 4, 3, 2, 3, 4, 6, 7, 8, 9, 7, 6, 5, 4, 3, 2, 1, 9, 8, 6,
        7, 8, 9, 6, 8, 6, 7, 8, 9, 7,
    ],
    [
        8, 9, 4, 9, 7, 6, 5, 4, 3, 4, 5, 7, 7, 9, 3, 4, 5, 6, 7, 9, 3, 4, 6, 9, 8, 7, 6, 5, 4, 6,
        7, 8, 9, 9, 6, 5, 4, 5, 6, 9, 9, 9, 8, 9, 9, 7, 5, 4, 3, 4, 6, 7, 9, 8, 7, 7, 6, 7, 8, 9,
        6, 5, 6, 7, 8, 9, 9, 9, 9, 5, 4, 3, 2, 1, 2, 5, 6, 9, 8, 9, 8, 8, 6, 5, 5, 3, 9, 8, 9, 8,
        8, 9, 8, 7, 8, 9, 8, 9, 8, 6,
    ],
    [
        9, 9, 3, 9, 8, 7, 6, 6, 5, 7, 6, 8, 9, 8, 1, 0, 1, 2, 8, 9, 5, 5, 9, 8, 7, 6, 5, 6, 3, 5,
        6, 7, 8, 9, 5, 4, 3, 4, 9, 8, 7, 6, 6, 7, 8, 9, 6, 7, 6, 5, 7, 9, 8, 7, 6, 4, 5, 6, 7, 8,
        9, 4, 5, 9, 9, 2, 9, 8, 7, 6, 3, 2, 1, 0, 1, 4, 5, 6, 7, 8, 9, 9, 7, 8, 9, 9, 9, 7, 6, 9,
        9, 8, 9, 8, 9, 3, 9, 3, 4, 5,
    ],
    [
        9, 8, 9, 9, 9, 8, 9, 7, 6, 9, 7, 9, 8, 7, 5, 2, 3, 7, 9, 9, 6, 9, 8, 7, 6, 5, 4, 3, 2, 4,
        5, 6, 7, 8, 9, 3, 2, 3, 9, 8, 7, 5, 4, 5, 9, 6, 9, 8, 7, 6, 8, 9, 9, 9, 6, 5, 6, 7, 9, 9,
        2, 3, 9, 8, 9, 1, 0, 9, 8, 7, 4, 3, 2, 3, 2, 3, 4, 5, 8, 9, 5, 9, 8, 9, 9, 8, 7, 8, 5, 9,
        8, 7, 8, 9, 3, 2, 0, 1, 2, 3,
    ],
    [
        8, 7, 8, 9, 9, 9, 9, 8, 7, 8, 9, 8, 7, 6, 4, 3, 4, 6, 7, 8, 9, 3, 9, 8, 5, 4, 3, 2, 1, 3,
        4, 6, 9, 9, 2, 1, 0, 9, 8, 7, 6, 4, 3, 1, 3, 5, 8, 9, 8, 7, 9, 2, 9, 8, 7, 6, 7, 8, 9, 3,
        1, 9, 8, 7, 9, 3, 4, 5, 9, 8, 8, 9, 3, 4, 3, 4, 5, 6, 7, 9, 4, 5, 9, 2, 1, 9, 6, 5, 4, 9,
        8, 6, 7, 8, 9, 2, 1, 9, 3, 9,
    ],
    [
        4, 6, 7, 8, 9, 9, 9, 9, 8, 9, 5, 9, 8, 6, 5, 4, 6, 8, 8, 9, 1, 2, 3, 9, 6, 5, 4, 3, 2, 4,
        5, 9, 6, 4, 3, 2, 9, 8, 7, 6, 5, 3, 2, 0, 2, 3, 7, 8, 9, 9, 1, 0, 1, 9, 9, 7, 8, 9, 5, 4,
        9, 8, 7, 6, 8, 9, 9, 7, 9, 9, 7, 6, 4, 5, 4, 5, 6, 7, 8, 9, 5, 6, 8, 9, 9, 8, 7, 6, 3, 2,
        4, 5, 6, 7, 9, 3, 9, 8, 9, 8,
    ],
    [
        3, 4, 5, 6, 7, 8, 9, 8, 9, 5, 4, 2, 9, 7, 6, 5, 6, 7, 8, 9, 0, 1, 9, 8, 7, 6, 5, 6, 4, 5,
        7, 8, 9, 5, 4, 3, 5, 9, 9, 5, 4, 3, 2, 1, 3, 4, 5, 6, 7, 8, 9, 1, 2, 4, 9, 8, 9, 7, 6, 9,
        8, 9, 6, 5, 7, 8, 8, 9, 8, 9, 8, 7, 9, 6, 5, 9, 7, 8, 9, 9, 6, 8, 9, 9, 8, 7, 6, 5, 4, 1,
        2, 6, 7, 9, 8, 9, 8, 7, 9, 7,
    ],
    [
        4, 5, 9, 7, 9, 9, 8, 7, 6, 6, 2, 1, 9, 8, 7, 6, 7, 8, 9, 7, 9, 9, 9, 9, 9, 7, 8, 6, 5, 7,
        8, 9, 8, 7, 5, 5, 9, 8, 7, 6, 5, 4, 3, 2, 6, 5, 6, 7, 8, 9, 4, 3, 4, 5, 9, 9, 9, 8, 9, 8,
        7, 7, 5, 4, 5, 5, 7, 6, 7, 8, 9, 8, 8, 7, 6, 9, 8, 9, 9, 8, 7, 9, 8, 7, 9, 8, 7, 6, 5, 4,
        3, 4, 9, 8, 7, 7, 6, 6, 5, 6,
    ],
    [
        6, 7, 8, 9, 1, 0, 9, 8, 5, 4, 1, 0, 1, 9, 8, 9, 8, 9, 5, 6, 9, 8, 9, 9, 9, 8, 9, 8, 6, 8,
        9, 4, 9, 9, 6, 7, 8, 9, 8, 7, 6, 6, 5, 3, 4, 5, 7, 8, 9, 7, 5, 9, 5, 9, 8, 7, 8, 9, 8, 7,
        6, 5, 4, 3, 2, 3, 4, 5, 6, 7, 9, 9, 9, 8, 9, 9, 9, 0, 1, 9, 9, 9, 7, 6, 8, 9, 8, 7, 6, 5,
        6, 9, 8, 9, 6, 5, 4, 2, 3, 5,
    ],
    [
        7, 8, 9, 9, 2, 9, 8, 9, 4, 3, 2, 1, 2, 3, 9, 9, 9, 2, 3, 9, 8, 7, 9, 9, 7, 9, 9, 9, 9, 9,
        5, 3, 9, 8, 9, 8, 9, 2, 9, 8, 8, 8, 7, 9, 6, 9, 8, 9, 9, 8, 9, 8, 9, 9, 9, 5, 6, 7, 9, 8,
        7, 4, 3, 2, 1, 2, 3, 4, 5, 9, 8, 7, 7, 9, 8, 9, 2, 1, 9, 8, 7, 8, 9, 5, 7, 8, 9, 8, 7, 6,
        9, 8, 7, 6, 5, 4, 3, 1, 2, 3,
    ],
    [
        8, 9, 8, 7, 9, 8, 7, 6, 5, 4, 4, 3, 4, 4, 5, 9, 4, 3, 9, 8, 7, 6, 7, 8, 6, 7, 8, 9, 8, 8,
        9, 9, 8, 7, 8, 9, 9, 1, 9, 9, 9, 9, 9, 8, 7, 8, 9, 7, 8, 9, 9, 7, 8, 9, 8, 6, 8, 9, 9, 8,
        9, 7, 3, 1, 0, 3, 6, 7, 6, 9, 7, 6, 5, 6, 7, 8, 9, 9, 8, 7, 6, 7, 9, 4, 2, 1, 0, 9, 8, 7,
        9, 9, 8, 8, 6, 5, 4, 2, 3, 4,
    ],
    [
        9, 5, 6, 6, 7, 9, 8, 7, 6, 5, 6, 4, 5, 7, 9, 8, 9, 9, 8, 7, 6, 5, 3, 4, 5, 6, 9, 8, 7, 7,
        8, 6, 5, 6, 7, 9, 8, 9, 8, 7, 8, 7, 8, 9, 8, 9, 7, 6, 9, 8, 7, 6, 9, 9, 8, 7, 9, 9, 8, 7,
        6, 5, 4, 5, 6, 4, 5, 9, 9, 8, 9, 5, 4, 5, 6, 7, 8, 9, 6, 6, 5, 9, 9, 5, 6, 2, 1, 2, 9, 8,
        9, 9, 9, 9, 9, 8, 6, 6, 4, 6,
    ],
    [
        4, 3, 4, 5, 8, 9, 9, 8, 7, 6, 7, 5, 6, 9, 8, 7, 8, 9, 9, 8, 7, 3, 2, 3, 4, 5, 6, 9, 6, 5,
        6, 5, 4, 6, 8, 9, 7, 9, 7, 6, 9, 6, 7, 8, 9, 7, 6, 5, 4, 6, 4, 5, 7, 8, 9, 8, 9, 3, 9, 8,
        7, 6, 7, 6, 7, 8, 9, 9, 8, 7, 9, 4, 3, 4, 5, 6, 9, 6, 4, 3, 4, 8, 8, 9, 7, 3, 2, 3, 4, 9,
        9, 8, 5, 4, 5, 9, 8, 7, 5, 8,
    ],
    [
        3, 2, 3, 4, 9, 9, 8, 9, 8, 9, 9, 6, 7, 9, 8, 6, 7, 8, 9, 9, 8, 2, 1, 2, 6, 9, 9, 7, 4, 3,
        5, 1, 3, 5, 4, 5, 6, 8, 9, 5, 4, 5, 6, 9, 8, 6, 5, 4, 3, 2, 3, 4, 6, 7, 8, 9, 0, 1, 9, 9,
        8, 7, 8, 7, 8, 9, 9, 8, 7, 6, 5, 4, 2, 5, 4, 6, 8, 9, 3, 2, 3, 7, 7, 8, 9, 4, 3, 4, 9, 9,
        9, 7, 4, 3, 2, 1, 9, 8, 6, 9,
    ],
    [
        9, 0, 9, 9, 7, 6, 7, 8, 9, 8, 9, 9, 9, 8, 6, 5, 6, 8, 9, 3, 2, 1, 0, 1, 9, 8, 7, 6, 5, 2,
        1, 0, 1, 2, 3, 6, 7, 8, 9, 2, 3, 5, 6, 7, 9, 8, 6, 3, 2, 1, 2, 6, 7, 7, 8, 9, 1, 9, 8, 7,
        9, 8, 9, 8, 9, 9, 8, 9, 9, 7, 6, 2, 1, 2, 3, 8, 9, 6, 4, 3, 4, 5, 6, 7, 8, 9, 5, 6, 8, 9,
        8, 6, 5, 4, 3, 0, 9, 8, 7, 8,
    ],
    [
        8, 9, 8, 7, 6, 5, 2, 9, 6, 7, 9, 8, 9, 7, 5, 4, 5, 7, 8, 9, 9, 3, 9, 9, 8, 7, 6, 5, 4, 3,
        2, 7, 6, 3, 7, 7, 8, 9, 0, 1, 2, 5, 6, 9, 8, 9, 3, 2, 1, 0, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6,
        5, 9, 0, 9, 9, 8, 7, 8, 9, 8, 7, 3, 0, 1, 4, 5, 6, 9, 5, 4, 7, 6, 7, 8, 9, 8, 6, 7, 9, 5,
        9, 7, 6, 5, 4, 1, 2, 9, 8, 9,
    ],
    [
        7, 8, 9, 6, 5, 4, 3, 4, 5, 9, 8, 7, 6, 5, 4, 3, 4, 6, 9, 7, 8, 9, 8, 7, 9, 8, 7, 6, 7, 4,
        3, 4, 5, 4, 6, 7, 8, 9, 9, 4, 3, 4, 9, 8, 7, 6, 5, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 7,
        3, 2, 1, 9, 8, 7, 6, 7, 9, 9, 4, 2, 1, 2, 3, 4, 6, 8, 9, 5, 8, 7, 8, 9, 1, 9, 7, 9, 0, 3,
        9, 8, 7, 6, 5, 6, 7, 8, 9, 9,
    ],
    [
        6, 9, 8, 7, 6, 9, 9, 9, 6, 7, 9, 8, 5, 4, 3, 2, 3, 7, 9, 6, 9, 9, 8, 6, 7, 9, 9, 7, 8, 5,
        4, 5, 7, 5, 7, 8, 9, 6, 8, 9, 4, 6, 7, 9, 8, 7, 9, 4, 4, 3, 5, 4, 5, 6, 9, 8, 9, 9, 8, 9,
        4, 3, 9, 8, 7, 6, 5, 9, 8, 7, 6, 3, 2, 3, 4, 5, 8, 9, 8, 7, 9, 9, 9, 1, 0, 9, 8, 9, 1, 2,
        3, 9, 8, 7, 6, 7, 9, 9, 8, 8,
    ],
    [
        5, 4, 9, 8, 9, 8, 7, 8, 9, 9, 8, 7, 6, 5, 4, 1, 2, 3, 4, 5, 7, 8, 9, 5, 6, 7, 9, 8, 9, 6,
        5, 6, 7, 6, 7, 9, 3, 5, 7, 8, 9, 8, 9, 0, 9, 9, 8, 9, 5, 4, 6, 5, 6, 7, 8, 9, 7, 6, 7, 8,
        9, 9, 8, 7, 6, 5, 4, 5, 9, 8, 9, 4, 3, 6, 5, 6, 7, 8, 9, 8, 9, 9, 3, 2, 1, 9, 9, 5, 4, 3,
        4, 5, 9, 8, 9, 8, 9, 7, 6, 7,
    ],
    [
        6, 5, 8, 9, 8, 7, 6, 9, 9, 9, 9, 9, 7, 6, 7, 2, 3, 4, 5, 6, 8, 9, 5, 4, 5, 8, 9, 9, 8, 7,
        7, 9, 8, 9, 8, 9, 2, 3, 4, 5, 8, 9, 3, 1, 3, 9, 7, 9, 9, 8, 7, 6, 8, 8, 9, 7, 6, 5, 6, 9,
        8, 9, 9, 8, 8, 7, 3, 4, 5, 9, 6, 5, 8, 7, 6, 9, 8, 9, 8, 9, 5, 7, 9, 4, 9, 8, 7, 6, 5, 6,
        5, 7, 8, 9, 3, 9, 7, 6, 5, 6,
    ],
    [
        7, 6, 7, 8, 9, 6, 5, 6, 9, 8, 9, 7, 6, 5, 4, 3, 4, 5, 6, 7, 8, 9, 3, 2, 3, 4, 5, 9, 9, 8,
        8, 9, 9, 9, 9, 4, 3, 4, 5, 6, 7, 8, 9, 2, 9, 8, 6, 8, 9, 9, 8, 9, 9, 9, 7, 6, 5, 4, 5, 6,
        7, 9, 9, 9, 7, 6, 4, 5, 6, 9, 8, 7, 9, 8, 7, 8, 9, 9, 7, 7, 4, 5, 6, 9, 8, 9, 8, 9, 8, 7,
        6, 8, 9, 9, 2, 9, 8, 7, 4, 3,
    ],
    [
        9, 8, 9, 9, 8, 9, 4, 7, 9, 7, 8, 9, 7, 7, 6, 5, 6, 8, 9, 8, 9, 3, 3, 1, 2, 5, 6, 8, 9, 9,
        9, 5, 9, 8, 7, 6, 4, 5, 6, 7, 8, 9, 9, 9, 8, 7, 5, 7, 8, 9, 9, 3, 2, 9, 8, 7, 8, 5, 9, 7,
        9, 8, 9, 8, 9, 7, 5, 6, 7, 8, 9, 8, 9, 9, 8, 9, 9, 8, 6, 5, 3, 4, 9, 8, 7, 8, 9, 9, 9, 8,
        7, 9, 9, 8, 9, 9, 9, 6, 5, 2,
    ],
    [
        4, 9, 8, 7, 6, 4, 3, 6, 5, 6, 7, 9, 8, 9, 7, 6, 7, 9, 3, 9, 3, 2, 1, 0, 1, 4, 7, 8, 9, 6,
        5, 3, 2, 9, 9, 9, 8, 7, 8, 9, 9, 9, 9, 8, 7, 6, 4, 5, 9, 9, 9, 9, 1, 0, 9, 9, 9, 9, 8, 9,
        8, 7, 6, 7, 8, 9, 9, 7, 8, 9, 6, 9, 4, 4, 9, 9, 9, 9, 7, 9, 2, 9, 8, 7, 6, 7, 9, 9, 9, 9,
        9, 8, 9, 7, 8, 9, 8, 9, 2, 1,
    ],
    [
        3, 2, 9, 8, 4, 3, 2, 3, 4, 5, 7, 8, 9, 9, 8, 9, 8, 9, 2, 9, 5, 3, 2, 1, 2, 3, 7, 9, 6, 5,
        4, 2, 1, 9, 8, 9, 9, 8, 9, 9, 8, 7, 6, 4, 3, 2, 3, 6, 7, 8, 9, 8, 9, 1, 9, 8, 7, 8, 7, 9,
        8, 7, 5, 6, 9, 9, 8, 9, 9, 9, 5, 3, 2, 3, 7, 8, 9, 9, 9, 8, 9, 7, 6, 5, 5, 6, 7, 8, 9, 9,
        8, 7, 8, 6, 9, 8, 7, 8, 9, 0,
    ],
    [
        5, 0, 9, 9, 3, 2, 1, 2, 3, 4, 8, 7, 7, 8, 9, 5, 9, 8, 9, 8, 9, 4, 3, 2, 4, 5, 6, 9, 7, 9,
        5, 4, 9, 9, 7, 8, 9, 9, 9, 9, 9, 8, 7, 3, 2, 1, 4, 5, 8, 9, 8, 7, 8, 9, 9, 9, 5, 7, 6, 5,
        4, 3, 4, 6, 8, 9, 7, 8, 9, 8, 9, 2, 1, 5, 6, 9, 9, 9, 8, 7, 8, 9, 7, 4, 3, 2, 3, 5, 6, 9,
        9, 6, 7, 4, 8, 7, 6, 7, 9, 1,
    ],
    [
        6, 9, 8, 8, 4, 3, 0, 1, 2, 3, 4, 5, 6, 7, 9, 4, 5, 7, 8, 7, 8, 9, 4, 5, 6, 6, 7, 8, 9, 8,
        9, 9, 8, 7, 6, 7, 8, 9, 9, 8, 7, 6, 5, 4, 3, 2, 5, 6, 7, 8, 9, 6, 9, 9, 9, 8, 4, 3, 2, 1,
        0, 1, 2, 3, 4, 5, 6, 9, 8, 7, 8, 9, 2, 4, 5, 9, 9, 7, 6, 6, 7, 8, 9, 5, 4, 3, 4, 6, 9, 8,
        6, 5, 5, 3, 2, 8, 5, 8, 9, 2,
    ],
    [
        7, 9, 7, 6, 6, 5, 1, 2, 3, 7, 8, 9, 7, 9, 4, 3, 4, 4, 5, 6, 7, 8, 9, 6, 7, 8, 8, 9, 9, 7,
        8, 9, 7, 6, 5, 8, 7, 8, 9, 9, 8, 7, 6, 5, 4, 3, 6, 7, 8, 9, 6, 5, 6, 9, 8, 7, 5, 4, 9, 4,
        2, 4, 3, 4, 5, 6, 7, 9, 3, 5, 7, 9, 3, 5, 9, 8, 7, 6, 5, 5, 6, 9, 8, 9, 9, 4, 5, 7, 9, 9,
        6, 4, 4, 2, 1, 5, 4, 5, 8, 9,
    ],
    [
        8, 9, 9, 5, 4, 3, 2, 4, 5, 6, 7, 8, 9, 3, 2, 1, 2, 3, 4, 5, 6, 7, 9, 7, 8, 9, 9, 8, 7, 6,
        7, 8, 9, 4, 3, 5, 6, 7, 8, 9, 9, 9, 7, 6, 5, 6, 7, 8, 9, 2, 1, 4, 5, 6, 9, 8, 6, 9, 8, 9,
        3, 5, 4, 5, 6, 7, 8, 9, 2, 3, 9, 8, 9, 7, 8, 9, 8, 7, 4, 3, 4, 6, 7, 7, 8, 9, 6, 7, 9, 8,
        4, 3, 2, 1, 0, 2, 3, 4, 7, 8,
    ],
    [
        9, 9, 8, 7, 6, 5, 9, 5, 6, 7, 8, 9, 5, 4, 3, 2, 3, 4, 5, 6, 8, 9, 9, 8, 9, 9, 9, 9, 8, 4,
        6, 7, 8, 9, 2, 6, 7, 8, 9, 9, 8, 9, 8, 7, 6, 7, 8, 9, 2, 1, 0, 3, 4, 8, 9, 9, 9, 8, 7, 8,
        9, 6, 7, 8, 9, 8, 9, 0, 1, 9, 8, 7, 8, 9, 9, 9, 9, 9, 5, 2, 3, 4, 5, 6, 9, 9, 7, 9, 8, 7,
        6, 4, 6, 5, 4, 3, 4, 5, 6, 7,
    ],
    [
        2, 1, 9, 8, 9, 6, 8, 7, 8, 9, 9, 7, 6, 5, 6, 3, 4, 5, 6, 7, 9, 2, 1, 9, 9, 8, 9, 9, 6, 5,
        6, 8, 9, 5, 4, 5, 8, 9, 8, 6, 7, 8, 9, 8, 7, 8, 9, 4, 3, 2, 1, 2, 5, 6, 7, 9, 8, 7, 6, 5,
        9, 9, 8, 9, 3, 9, 2, 1, 9, 8, 9, 6, 5, 4, 8, 7, 8, 9, 3, 1, 2, 3, 4, 7, 9, 9, 8, 9, 9, 8,
        7, 5, 6, 6, 5, 4, 5, 6, 7, 8,
    ],
    [
        9, 0, 9, 9, 8, 7, 9, 9, 9, 9, 9, 9, 8, 6, 5, 4, 5, 6, 7, 9, 8, 9, 2, 9, 9, 7, 9, 8, 7, 6,
        7, 9, 7, 6, 5, 6, 9, 8, 7, 5, 8, 9, 7, 9, 8, 9, 6, 5, 4, 3, 4, 3, 4, 5, 9, 8, 7, 6, 5, 4,
        8, 9, 9, 0, 1, 2, 9, 9, 8, 7, 6, 5, 4, 3, 5, 6, 9, 8, 9, 2, 3, 5, 6, 8, 9, 4, 9, 5, 6, 9,
        8, 9, 9, 8, 6, 7, 6, 7, 9, 9,
    ],
    [
        8, 9, 8, 9, 9, 9, 9, 9, 8, 7, 7, 6, 9, 7, 6, 5, 6, 7, 9, 8, 7, 8, 9, 8, 7, 6, 4, 9, 8, 7,
        8, 9, 8, 7, 6, 7, 9, 5, 6, 4, 4, 5, 6, 8, 9, 9, 8, 6, 8, 7, 6, 4, 5, 6, 7, 9, 8, 2, 1, 3,
        7, 8, 9, 2, 4, 9, 8, 9, 7, 5, 4, 3, 1, 2, 4, 5, 6, 7, 8, 9, 4, 9, 7, 9, 4, 3, 2, 3, 4, 9,
        9, 9, 9, 8, 7, 8, 7, 8, 9, 1,
    ],
    [
        7, 6, 6, 8, 8, 9, 9, 8, 7, 6, 5, 4, 5, 9, 8, 9, 8, 9, 9, 7, 6, 8, 9, 9, 9, 5, 3, 4, 9, 8,
        9, 9, 9, 8, 7, 8, 9, 4, 3, 2, 3, 4, 5, 6, 8, 9, 9, 9, 9, 8, 7, 9, 7, 7, 9, 8, 7, 3, 2, 5,
        6, 8, 9, 9, 9, 8, 7, 6, 5, 3, 2, 1, 0, 1, 3, 4, 5, 6, 7, 8, 9, 8, 9, 9, 9, 4, 6, 7, 9, 8,
        7, 8, 9, 9, 8, 9, 8, 9, 9, 0,
    ],
    [
        5, 4, 5, 6, 7, 8, 9, 9, 9, 4, 3, 2, 3, 4, 9, 9, 9, 9, 8, 6, 5, 7, 8, 9, 8, 7, 4, 5, 6, 9,
        9, 9, 7, 9, 8, 9, 5, 4, 3, 1, 9, 5, 6, 7, 9, 8, 9, 8, 9, 9, 8, 9, 8, 9, 8, 6, 5, 4, 5, 6,
        8, 9, 9, 8, 7, 9, 8, 9, 7, 4, 3, 2, 1, 2, 6, 5, 6, 7, 8, 9, 8, 7, 9, 9, 8, 9, 7, 9, 8, 8,
        6, 7, 8, 9, 9, 7, 9, 7, 8, 9,
    ],
    [
        4, 2, 4, 5, 7, 9, 0, 2, 9, 5, 9, 3, 4, 5, 8, 9, 8, 7, 6, 5, 4, 6, 7, 8, 9, 9, 7, 6, 7, 8,
        9, 8, 6, 5, 9, 7, 6, 5, 9, 9, 8, 9, 7, 9, 9, 7, 8, 7, 8, 9, 9, 8, 9, 3, 9, 7, 6, 5, 9, 7,
        8, 9, 8, 9, 6, 5, 9, 7, 6, 5, 4, 3, 2, 3, 4, 7, 8, 8, 9, 9, 8, 6, 9, 8, 7, 8, 9, 8, 7, 6,
        5, 6, 9, 9, 8, 6, 5, 6, 7, 8,
    ],
    [
        2, 1, 3, 4, 8, 9, 1, 9, 8, 9, 8, 9, 5, 6, 7, 8, 9, 9, 6, 5, 3, 4, 5, 7, 8, 9, 9, 9, 8, 9,
        8, 6, 5, 4, 9, 8, 7, 9, 8, 8, 7, 8, 9, 8, 9, 6, 7, 6, 7, 8, 9, 7, 6, 5, 9, 8, 7, 7, 8, 9,
        9, 6, 7, 8, 9, 4, 2, 9, 8, 6, 5, 4, 3, 6, 5, 6, 7, 9, 8, 7, 6, 5, 9, 7, 6, 7, 8, 9, 6, 5,
        4, 5, 9, 9, 6, 5, 4, 5, 8, 9,
    ],
    [
        3, 2, 5, 6, 7, 8, 9, 9, 7, 8, 7, 8, 9, 9, 8, 9, 9, 8, 7, 3, 2, 1, 3, 6, 7, 8, 9, 9, 9, 8,
        7, 8, 4, 3, 4, 9, 9, 8, 7, 5, 6, 9, 9, 7, 6, 5, 6, 5, 6, 9, 9, 8, 7, 9, 8, 9, 9, 8, 9, 3,
        6, 5, 6, 7, 8, 9, 0, 1, 9, 7, 6, 6, 5, 6, 8, 9, 8, 9, 7, 6, 7, 4, 7, 6, 5, 6, 7, 8, 9, 6,
        3, 6, 8, 8, 9, 1, 2, 3, 4, 5,
    ],
    [
        9, 3, 4, 6, 7, 8, 9, 8, 6, 5, 6, 7, 8, 9, 9, 9, 8, 6, 5, 4, 7, 5, 4, 5, 6, 7, 8, 9, 9, 9,
        6, 5, 1, 2, 5, 6, 9, 7, 6, 4, 5, 9, 8, 6, 5, 4, 5, 4, 6, 7, 8, 9, 9, 8, 7, 6, 5, 9, 1, 2,
        3, 4, 8, 9, 9, 2, 1, 2, 9, 8, 7, 7, 8, 8, 9, 5, 9, 9, 9, 5, 4, 3, 1, 2, 4, 7, 8, 9, 7, 5,
        4, 5, 6, 7, 9, 0, 1, 2, 3, 4,
    ],
    [
        8, 9, 5, 7, 8, 9, 8, 6, 5, 4, 5, 6, 7, 9, 2, 1, 9, 7, 6, 9, 8, 6, 6, 6, 7, 8, 9, 6, 9, 8,
        9, 4, 2, 3, 4, 9, 8, 8, 4, 3, 6, 9, 9, 5, 4, 3, 2, 3, 4, 6, 7, 8, 9, 7, 6, 5, 4, 3, 2, 3,
        4, 5, 9, 5, 4, 3, 2, 5, 6, 9, 9, 8, 9, 9, 5, 4, 3, 9, 8, 7, 5, 1, 0, 1, 2, 6, 7, 8, 9, 6,
        5, 6, 7, 9, 8, 9, 2, 9, 4, 5,
    ],
    [
        7, 8, 9, 9, 9, 8, 7, 6, 5, 3, 4, 5, 9, 8, 9, 2, 9, 8, 8, 9, 8, 7, 7, 7, 8, 9, 6, 5, 9, 7,
        8, 9, 3, 4, 9, 8, 7, 6, 5, 4, 9, 8, 5, 4, 3, 2, 1, 2, 4, 5, 9, 9, 8, 9, 7, 9, 6, 4, 5, 4,
        5, 6, 7, 9, 8, 4, 3, 4, 5, 6, 7, 9, 8, 7, 4, 3, 2, 9, 7, 5, 3, 2, 1, 5, 4, 5, 6, 7, 8, 9,
        6, 7, 9, 8, 7, 8, 9, 8, 9, 6,
    ],
    [
        6, 8, 9, 9, 9, 9, 9, 5, 4, 2, 3, 5, 6, 7, 8, 9, 8, 9, 9, 2, 9, 9, 9, 8, 9, 7, 5, 4, 5, 6,
        7, 8, 9, 9, 9, 9, 9, 7, 6, 9, 8, 7, 6, 5, 6, 3, 4, 6, 5, 7, 8, 9, 7, 8, 9, 8, 7, 5, 6, 5,
        7, 7, 8, 9, 6, 5, 4, 5, 7, 7, 8, 9, 7, 6, 5, 9, 3, 4, 9, 8, 5, 4, 3, 6, 5, 7, 8, 9, 9, 4,
        9, 8, 9, 7, 6, 7, 9, 7, 8, 9,
    ],
    [
        5, 6, 7, 8, 9, 8, 7, 6, 0, 1, 2, 9, 8, 9, 9, 7, 7, 8, 9, 1, 2, 4, 5, 9, 6, 6, 4, 3, 4, 5,
        4, 5, 9, 8, 7, 8, 9, 9, 8, 9, 9, 9, 7, 6, 5, 4, 5, 6, 7, 8, 9, 8, 6, 7, 9, 9, 8, 6, 7, 6,
        8, 8, 9, 8, 7, 6, 5, 6, 8, 9, 9, 9, 9, 7, 9, 8, 9, 9, 8, 7, 6, 8, 4, 5, 6, 8, 9, 5, 4, 3,
        5, 9, 8, 9, 5, 6, 7, 6, 7, 8,
    ],
    [
        4, 5, 6, 7, 9, 9, 8, 7, 3, 2, 3, 4, 9, 7, 6, 5, 6, 9, 1, 0, 1, 5, 9, 7, 5, 4, 3, 2, 1, 2,
        3, 9, 8, 7, 6, 6, 7, 8, 9, 5, 3, 2, 9, 7, 6, 5, 6, 7, 8, 9, 7, 6, 5, 8, 8, 9, 8, 7, 8, 7,
        8, 9, 9, 9, 8, 7, 6, 9, 9, 8, 8, 9, 8, 9, 8, 7, 6, 4, 9, 8, 7, 9, 7, 6, 8, 9, 2, 9, 7, 5,
        9, 8, 7, 8, 4, 5, 4, 5, 6, 9,
    ],
    [
        3, 4, 7, 9, 8, 7, 6, 5, 4, 5, 4, 5, 8, 9, 5, 4, 5, 8, 9, 1, 2, 9, 8, 9, 6, 5, 4, 3, 2, 3,
        9, 8, 6, 5, 4, 5, 6, 7, 8, 9, 4, 3, 9, 8, 7, 8, 8, 9, 9, 6, 4, 3, 4, 5, 6, 7, 9, 8, 9, 8,
        9, 9, 8, 9, 9, 8, 9, 8, 7, 6, 7, 8, 7, 9, 7, 6, 5, 3, 4, 9, 9, 9, 8, 7, 8, 9, 0, 9, 9, 9,
        8, 7, 6, 4, 3, 4, 3, 4, 5, 7,
    ],
    [
        2, 5, 6, 7, 9, 8, 7, 6, 5, 7, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 9, 8, 7, 8, 9, 6, 5, 4, 4, 9,
        7, 6, 5, 4, 3, 4, 5, 6, 7, 8, 9, 4, 5, 9, 8, 9, 9, 9, 8, 5, 3, 2, 1, 4, 5, 7, 9, 9, 0, 9,
        8, 7, 6, 7, 8, 9, 8, 7, 6, 5, 4, 7, 6, 8, 9, 9, 7, 4, 5, 6, 9, 9, 9, 8, 9, 6, 9, 8, 9, 9,
        9, 8, 7, 3, 2, 3, 2, 5, 6, 5,
    ],
    [
        3, 4, 5, 6, 7, 9, 8, 9, 7, 8, 6, 7, 8, 9, 7, 6, 8, 8, 9, 7, 6, 5, 6, 7, 8, 9, 6, 7, 9, 8,
        6, 6, 4, 3, 2, 3, 4, 7, 8, 9, 6, 5, 6, 7, 9, 3, 9, 8, 7, 6, 4, 3, 2, 3, 4, 6, 8, 9, 1, 9,
        8, 6, 5, 6, 9, 8, 7, 6, 5, 4, 3, 6, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 9, 9, 5, 4, 6, 6, 9, 8,
        7, 6, 5, 2, 1, 0, 1, 2, 3, 4,
    ],
    [
        4, 5, 6, 8, 9, 9, 9, 9, 8, 9, 8, 8, 9, 9, 8, 7, 9, 9, 6, 5, 5, 4, 7, 7, 9, 8, 7, 9, 8, 6,
        5, 4, 3, 2, 1, 2, 3, 8, 9, 8, 7, 6, 7, 8, 9, 2, 1, 9, 8, 7, 5, 4, 3, 6, 5, 6, 9, 9, 9, 8,
        7, 5, 4, 5, 9, 7, 6, 5, 4, 3, 2, 3, 4, 5, 7, 9, 9, 8, 7, 8, 9, 9, 9, 5, 4, 3, 4, 5, 6, 9,
        9, 8, 4, 3, 2, 3, 2, 3, 4, 5,
    ],
    [
        5, 6, 7, 9, 9, 8, 7, 6, 9, 1, 9, 9, 8, 9, 9, 8, 9, 6, 5, 4, 4, 3, 4, 5, 6, 9, 8, 9, 9, 7,
        6, 3, 2, 1, 0, 3, 4, 9, 9, 9, 8, 7, 8, 9, 4, 3, 5, 9, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 8, 7,
        6, 5, 2, 9, 8, 6, 5, 4, 3, 2, 1, 2, 3, 6, 8, 9, 2, 9, 8, 9, 8, 9, 8, 7, 5, 6, 7, 7, 8, 9,
        9, 8, 5, 4, 3, 4, 5, 4, 5, 6,
    ],
    [
        6, 7, 8, 9, 9, 9, 6, 5, 3, 2, 4, 6, 7, 8, 9, 9, 6, 5, 4, 3, 2, 1, 2, 6, 9, 8, 9, 0, 1, 9,
        5, 4, 3, 2, 1, 4, 6, 8, 9, 0, 9, 8, 9, 6, 5, 9, 6, 7, 9, 8, 8, 9, 9, 1, 9, 8, 9, 9, 9, 6,
        5, 4, 1, 9, 8, 7, 6, 5, 4, 3, 2, 5, 4, 5, 9, 0, 1, 2, 9, 6, 7, 8, 9, 8, 6, 7, 8, 9, 9, 9,
        8, 7, 6, 7, 9, 5, 7, 6, 8, 7,
    ],
    [
        7, 9, 9, 9, 9, 8, 6, 5, 4, 3, 4, 5, 6, 7, 9, 8, 7, 6, 7, 5, 3, 2, 3, 4, 5, 7, 8, 9, 9, 8,
        7, 5, 4, 3, 2, 5, 6, 9, 3, 2, 3, 9, 9, 7, 6, 7, 8, 9, 9, 9, 9, 5, 3, 2, 3, 9, 9, 9, 8, 7,
        6, 2, 0, 1, 9, 9, 8, 7, 5, 4, 3, 6, 5, 7, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9, 7, 8, 9, 0, 1, 2,
        9, 8, 7, 8, 9, 6, 8, 7, 9, 8,
    ],
];