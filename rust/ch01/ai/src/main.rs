/*
The code below is a result of asking Google Gemini to improve my existing code in `main.rs` to be more aligned with Rust's syntactical variations/sugar.
It interpreted my prompt as idiomatic Rust syntax and patterns.
Can't complain. :)
*/

fn bin_search(nums: &[i32], val: i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = nums.len();

    while low < high {
        let mid = (low + high) / 2;

        match nums.get(mid) {
            Some(&num) => {
                if num == val {
                    return Some(mid);
                } else if num > val {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            None => {
                return None;
            }
        }
    }
    None
}

fn main() {
    let nums: [i32; 100] = std::array::from_fn(|i| i as i32);

    match bin_search(&nums, 50) {
        Some(index) => println!("Found 50 at index: {}", index),
        None => println!("42 not found"),
    }

    match bin_search(&nums, 73) {
        Some(index) => println!("Found 73 at index: {}", index),
        None => println!("73 not found"),
    }

    match bin_search(&nums, 0) {
        Some(index) => println!("Found 0 at index: {}", index),
        None => println!("0 not found"),
    }

    match bin_search(&nums, -5) {
        Some(index) => println!("Found -5 at index: {}", index),
        None => println!("-5 not found"),
    }

    let empty_nums: [i32; 0] = [];
    match bin_search(&empty_nums, 3) {
        Some(index) => println!("Found 3 in empty slice at {}", index),
        None => println!("Couldn't find 3 in empty slice!"),
    }
}
