use std::io::{self, Write};

fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    print!("Target: ");
    io::stdout().flush().unwrap();
    let mut target = String::new();

    io::stdin().read_line(&mut target).unwrap();

    let target: i32 = target.trim().parse().unwrap();

    match linear_search(&nums, target) {
        Some(i) => println!("{}", i),
        None => println!("Target not found"),
    }
}
