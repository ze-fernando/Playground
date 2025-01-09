fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() as i32 - 1;

    while start <= end {
        let middle = (start + end) / 2;

        if arr[middle as usize] == target {
            return Some(middle as usize);
        } else if arr[middle as usize] < target {
            start = middle + 1;
        } else {
            end = middle - 1;
        }
    }
    None
}

fn main() {
    let nums = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 31, 32, 33, 34, 35,
        36, 37, 38, 39, 41,
    ];
    let target = 8; // You can change for tests

    match binary_search(&nums, target) {
        Some(i) => println!("Pos: {}", i),
        None => println!("Target not found"),
    }
}
