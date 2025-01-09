fn selection_sort(arr: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..arr.len(){
        let mut min = i;
        for j in i+1..arr.len(){
            if arr[j] < arr[min]{
                min = j;
            }
        }
        arr.swap(i, min);
    }
    arr
}

fn main() {
    let mut nums = vec![2, 5, 23, 91, 6, 7, 1, 57, 8, 33];

    println!("{:?}", selection_sort(&mut nums));
}