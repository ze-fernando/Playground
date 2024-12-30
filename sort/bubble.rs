fn bubble_sort(arr: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if arr[i] < arr[j] {
                arr.swap(j, i);
            }
        }
    }
    arr
}

fn main() {
    let mut nums = vec![2, 5, 23, 91, 6, 7, 1, 57, 8, 33];

    println!("{:?}", bubble_sort(&mut nums));
}
