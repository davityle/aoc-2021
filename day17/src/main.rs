fn main() {
    let valid_velocity_count = find_all_valid_velocities();
    println!("{:?}", valid_velocity_count);
}

fn find_all_valid_velocities() -> i32 {
    let mut valid_velocity_count = 0;
    let max_single_magnitude =
        (INPUT[1].0.abs() - INPUT[0].0.abs()) * (INPUT[1].1.abs() - INPUT[0].1.abs());

    for x in -max_single_magnitude..max_single_magnitude {
        for y in -max_single_magnitude..max_single_magnitude {
            let dir_vector = (x, y);

            match get_max_height(dir_vector) {
                Some(_) => valid_velocity_count += 1,
                None => {}
            }
        }
    }

    valid_velocity_count
}

fn get_max_height(mut dir_vector: (i32, i32)) -> Option<i32> {
    let mut position = (0, 0);
    let mut max_height = 0;

    while !is_in_box(position, INPUT) && position.0 < INPUT[1].0 && position.1 > INPUT[1].1 {
        position.0 += dir_vector.0;
        position.1 += dir_vector.1;

        if dir_vector.0 > 0 {
            dir_vector.0 -= 1;
        }
        dir_vector.1 -= 1;
        if position.1 > max_height {
            max_height = position.1;
        }
    }
    if !is_in_box(position, INPUT) {
        None
    } else {
        Some(max_height)
    }
}

fn is_in_box(point: (i32, i32), b: [(i32, i32); 2]) -> bool {
    b[0].0 <= point.0 && point.0 <= b[1].0 && b[0].1 >= point.1 && point.1 >= b[1].1
}

const _: [(i32, i32); 2] = [(20, -5), (30, -10)];
const INPUT: [(i32, i32); 2] = [(56, -134), (76, -162)];
