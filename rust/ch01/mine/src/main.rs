fn bin_search(nums: [i32; 100], n: i32) {
    let mut low = 0;
    let mut high = nums.len();

    while low <= high {
        let mid = (low + high) / 2;
        let num = nums[mid];
        if num == n {
            println!("{}", mid);
            return;
        } else if num > n {
            high = mid - 1;
        } else {
            low = mid;
        }
    }
}

fn main() {
    let mut nums: [i32; 100] = [0; 100];
    for i in 0..100 {
        nums[i] = i as i32;
    }

    bin_search(nums.clone(), 42);
    bin_search(nums.clone(), 81);
}
