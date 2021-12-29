use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

fn main() {
    let mut octopi = INPUT;
    let mut total_flashes: u64 = 0;
    let mut synced_step: u64 = 0;

    for step in 0..u64::MAX {
        for y in 0..octopi.len() {
            for x in 0..octopi[y].len() {
                octopi[y][x] = octopi[y][x] + 1;
            }
        }
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        for y in 0..octopi.len() {
            for x in 0..octopi[y].len() {
                flash(x, y, &mut octopi, &mut flashed);
            }
        }

        for &(x, y) in flashed.iter() {
            octopi[y][x] = 0;
        }

        total_flashes = total_flashes + flashed.len() as u64;

        if flashed.len() == X_CONSTRAINT * Y_CONSTRAINT {
            synced_step = step;
            break;
        }
    }

    println!("{}", total_flashes);
    println!("{}", synced_step + 1);
}

fn flash(
    x: usize,
    y: usize,
    octopi: &mut [[u8; X_CONSTRAINT]; Y_CONSTRAINT],
    flashed: &mut HashSet<(usize, usize)>,
) {
    if x >= X_CONSTRAINT || y >= Y_CONSTRAINT || flashed.contains(&(x, y)) || octopi[y][x] <= 9 {
        return;
    }
    flashed.insert((x, y));
    for_directions(x, y, |x, y| octopi[y][x] = octopi[y][x] + 1);
    for_directions(x, y, |x, y| flash(x, y, octopi, flashed));
}

fn for_directions<F>(x: usize, y: usize, mut f: F)
where
    F: FnMut(usize, usize) -> (),
{
    for (x_dir, y_dir) in DIRECTIONS {
        let new_x = x as i32 + x_dir;
        let new_y = y as i32 + y_dir;
        if new_x < 0
            || new_y < 0
            || new_x as usize >= X_CONSTRAINT
            || new_y as usize >= Y_CONSTRAINT
        {
            continue;
        }
        f(new_x as usize, new_y as usize);
    }
}

const X_CONSTRAINT: usize = 10;
const Y_CONSTRAINT: usize = 10;

const _: [[u8; X_CONSTRAINT]; Y_CONSTRAINT] = [
    [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
    [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
    [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
    [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
    [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
    [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
    [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
    [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
    [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
    [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
];

const INPUT: [[u8; X_CONSTRAINT]; Y_CONSTRAINT] = [
    [4, 7, 2, 1, 2, 2, 4, 6, 6, 3],
    [6, 8, 7, 5, 4, 1, 5, 2, 7, 6],
    [2, 7, 4, 2, 4, 4, 8, 4, 2, 8],
    [4, 8, 7, 8, 2, 3, 1, 5, 5, 6],
    [5, 6, 8, 4, 6, 4, 3, 7, 4, 3],
    [3, 5, 5, 3, 6, 8, 1, 8, 6, 6],
    [4, 7, 8, 8, 1, 8, 3, 6, 2, 5],
    [4, 2, 5, 5, 8, 5, 6, 5, 3, 2],
    [1, 4, 1, 5, 8, 1, 8, 7, 7, 5],
    [2, 3, 2, 6, 8, 8, 6, 1, 2, 5],
];
