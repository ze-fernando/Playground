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
    let target = 8; // You can change for tests

    match linear_search(&nums, target) {
        Some(i) => println!("Pos: {}", i),
        None => println!("Target not found"),
    }
}
