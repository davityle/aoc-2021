use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ptr::NonNull;

fn main() {
    let mut numbers: Vec<Value> = Vec::new();

    let lines = read_lines("./input.txt").unwrap();
    for line in lines {
        let number: Value = serde_json::from_str(line.unwrap().as_str()).unwrap();
        numbers.push(number);
    }

    let mut max_magnitude: u64 = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let mut num1 = create_tree_from_json(&numbers[i], None);
            let mut num2 = create_tree_from_json(&numbers[j], None);

            let addition_tree = match (&mut num1, &mut num2) {
                (SnailNumber::Pair(tree1), SnailNumber::Pair(tree2)) => {
                    let mut addition_tree = Box::new(SnailTree {
                        left: SnailNumber::Num(0, None),
                        right: SnailNumber::Num(0, None),
                        parent: None,
                    });
                    tree1.parent = Some(NonNull::from(addition_tree.as_mut()));
                    tree2.parent = Some(NonNull::from(addition_tree.as_mut()));
                    Some(addition_tree)
                }
                _ => None,
            };
            if let Some(mut tree) = addition_tree {
                tree.left = num1;
                tree.right = num2;
                let mut pair = SnailNumber::Pair(tree);
                while reduce_number(&mut pair) {}

                let magnitude = get_magnitude(&pair);
                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                }
            }
        }
    }

    println!("{}", max_magnitude);
}

fn get_magnitude(number: &SnailNumber) -> u64 {
    match number {
        SnailNumber::Num(v, _) => *v,
        SnailNumber::Pair(tree) => get_magnitude(&tree.left) * 3 + get_magnitude(&tree.right) * 2,
    }
}

fn reduce_number(number: &mut SnailNumber) -> bool {
    explode_number(number, 1) || split_number(number)
}

fn explode_number(number: &SnailNumber, depth: usize) -> bool {
    match number {
        SnailNumber::Num(_, _) => false,
        SnailNumber::Pair(tree) => {
            if depth <= 4 {
                explode_number(&tree.left, depth + 1) || explode_number(&tree.right, depth + 1)
            } else {
                if matches!(tree.left, SnailNumber::Num(_, _))
                    && matches!(tree.right, SnailNumber::Num(_, _))
                {
                    if let Some(mut parent_pointer) = tree.parent {
                        descend_to_first_num_left(
                            &mut parent_pointer,
                            &tree.left,
                            match tree.left {
                                SnailNumber::Num(n, _) => n,
                                _ => 0,
                            },
                        );
                        descend_to_first_num_right(
                            &mut parent_pointer,
                            &tree.right,
                            match tree.right {
                                SnailNumber::Num(n, _) => n,
                                _ => 0,
                            },
                        );
                    }
                    let mut grand_parent_pointer = tree.parent.unwrap();

                    let replace_left = if let SnailNumber::Pair(parent_match) =
                        &unsafe { grand_parent_pointer.as_mut() }.left
                    {
                        parent_match == tree
                    } else {
                        false
                    };
                    if replace_left {
                        unsafe { grand_parent_pointer.as_mut() }.left =
                            SnailNumber::Num(0, Some(grand_parent_pointer.clone()));
                    } else {
                        unsafe { grand_parent_pointer.as_mut() }.right =
                            SnailNumber::Num(0, Some(grand_parent_pointer.clone()));
                    }
                    true
                } else {
                    explode_number(&tree.left, depth + 1) || explode_number(&tree.right, depth + 1)
                }
            }
        }
    }
}

fn split_number(number: &SnailNumber) -> bool {
    match number {
        SnailNumber::Num(value, parent_opt) => {
            if *value < 10 {
                false
            } else {
                let mut parent = parent_opt.unwrap();
                let mut new_tree = Box::new(SnailTree {
                    left: SnailNumber::Num(0, None),
                    right: SnailNumber::Num(0, None),
                    parent: Some(parent.clone()),
                });
                let left_value = ((*value as f64) / 2.0).floor() as u64;
                let right_value = ((*value as f64) / 2.0).ceil() as u64;

                let new_left = SnailNumber::Num(left_value, Some(NonNull::from(new_tree.as_mut())));
                let new_right =
                    SnailNumber::Num(right_value, Some(NonNull::from(new_tree.as_mut())));

                new_tree.left = new_left;
                new_tree.right = new_right;

                let left = (unsafe { parent.as_ref() }.left) == *number;

                if left {
                    unsafe { parent.as_mut() }.left = SnailNumber::Pair(new_tree);
                } else {
                    unsafe { parent.as_mut() }.right = SnailNumber::Pair(new_tree);
                }

                true
            }
        }
        SnailNumber::Pair(tree) => split_number(&tree.left) || split_number(&tree.right),
    }
}

fn print_snail_number(num: &SnailNumber) -> String {
    match num {
        SnailNumber::Num(value, _) => value.to_string(),
        SnailNumber::Pair(tree) => {
            let mut s = "[".to_owned();
            s.push_str(print_snail_number(&tree.left).as_str());
            s.push_str(",");
            s.push_str(print_snail_number(&tree.right).as_str());
            s.push_str("]");
            s
        }
    }
}

fn descend_to_first_num_left(pointer: &mut NonNull<SnailTree>, from: &SnailNumber, number: u64) {
    let value = unsafe { pointer.as_mut() };
    match &mut value.left {
        SnailNumber::Num(num, _) => *num += number,
        SnailNumber::Pair(tree) => {
            if !is_ancestor_of(tree, from) {
                ascend_to_first_num_right(&mut value.left, number);
            } else if let Some(mut parent_pointer) = value.parent {
                descend_to_first_num_left(&mut parent_pointer, &value.left, number);
            }
        }
    }
}

fn is_ancestor_of(tree: &SnailTree, ancestor: &SnailNumber) -> bool {
    if ancestor == &tree.left || ancestor == &tree.right {
        true
    } else {
        if let SnailNumber::Pair(sub_tree) = &tree.left {
            if is_ancestor_of(sub_tree, ancestor) {
                return true;
            }
        }
        if let SnailNumber::Pair(sub_tree) = &tree.right {
            if is_ancestor_of(sub_tree, ancestor) {
                return true;
            }
        }
        false
    }
}

fn ascend_to_first_num_right(snail_num: &mut SnailNumber, number: u64) {
    match snail_num {
        SnailNumber::Num(num, _) => *num += number,
        SnailNumber::Pair(tree) => {
            ascend_to_first_num_right(&mut tree.right, number);
        }
    }
}

fn descend_to_first_num_right(pointer: &mut NonNull<SnailTree>, from: &SnailNumber, number: u64) {
    let value = unsafe { pointer.as_mut() };
    match &mut value.right {
        SnailNumber::Num(num, _) => *num += number,
        SnailNumber::Pair(tree) => {
            if !is_ancestor_of(tree, from) {
                ascend_to_first_num_left(&mut value.right, number);
            } else if let Some(mut parent_pointer) = value.parent {
                descend_to_first_num_right(&mut parent_pointer, &value.right, number);
            }
        }
    }
}

fn ascend_to_first_num_left(snail_num: &mut SnailNumber, number: u64) {
    match snail_num {
        SnailNumber::Num(num, _) => *num += number,
        SnailNumber::Pair(tree) => {
            ascend_to_first_num_left(&mut tree.left, number);
        }
    }
}

fn create_tree_from_json(json: &Value, snail_parent: Option<NonNull<SnailTree>>) -> SnailNumber {
    match json {
        Value::Number(n) => SnailNumber::Num(n.as_u64().unwrap(), snail_parent),
        Value::Array(a) => {
            let parent = snail_parent.as_ref().map(|rc| rc.clone());
            let mut snail_pair = Box::new(SnailTree {
                left: SnailNumber::Num(0, None),
                right: SnailNumber::Num(0, None),
                parent,
            });
            let left = create_tree_from_json(&a[0], Some(NonNull::from(snail_pair.as_mut())));
            let right = create_tree_from_json(&a[1], Some(NonNull::from(snail_pair.as_mut())));
            snail_pair.left = left;
            snail_pair.right = right;

            SnailNumber::Pair(snail_pair)
        }
        _ => SnailNumber::Num(0, None),
    }
}

#[derive(Debug, PartialEq)]
struct SnailTree {
    left: SnailNumber,
    right: SnailNumber,
    parent: Option<NonNull<SnailTree>>,
}

#[derive(Debug, PartialEq)]
enum SnailNumber {
    Num(u64, Option<NonNull<SnailTree>>),
    Pair(Box<SnailTree>),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
